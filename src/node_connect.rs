use bitcoincore_rpc::{Auth, Client};
use dotenv::dotenv;
use std::env;
// use blocktalk::connection::Connection;
// use blocktalk::BlockTalkError;
// use std::sync::Arc;

// #[tokio::main]
// async fn main() -> Result<(), BlockTalkError> {
/*
async fn connect_to_bitcoin_node() -> Result<Arc<Connection>, BlockTalkError> {
    let socket_path = "/path/to/bitcoin-node.sock";
    
    // Call the connect function, which returns an Arc<Connection>
    let connection = Connection::connect(socket_path).await?;
    connection
}
*/

// Update to use blocktalk

pub fn connect_to_bitcoin_node() -> Client {
    dotenv().ok();

    let rpc_user = env::var("RPC_USERNAME").expect("RPC_USERNAME must be set");
    let rpc_password = env::var("RPC_PASSWORD").expect("RPC_PASSWORD must be set");
    
    let base_url = env::var("RPC_URL").expect("RPC_PORT must be set");
    let rpc_port = env::var("RPC_PORT").expect("RPC_PORT must be set");

    let mut rpc_url = base_url;
    rpc_url.push_str(&rpc_port);
    
    Client::new(
        &rpc_url,
        Auth::UserPass(rpc_user, rpc_password)
    ).expect("Failed to create RPC client")
}
