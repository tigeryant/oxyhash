use bitcoincore_rpc::{Auth, Client};
use dotenv::dotenv;
use std::env;

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
