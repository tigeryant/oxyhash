use bitcoincore_rpc::json::GetBlockTemplateResult;
use bitcoincore_rpc::{Client, RpcApi};
use bitcoincore_rpc::bitcoincore_rpc_json::{
    GetBlockTemplateModes,
    GetBlockTemplateRules
};

// Remove this
pub fn get_mempool_transactions(client: &Client) -> Vec<String> {
    // Get raw mempool
    let txids = client.get_raw_mempool().expect("Failed to get mempool");
    txids.iter().map(|txid| txid.to_string()).collect()
}

pub fn get_block_template(client: &Client) -> GetBlockTemplateResult {
    let mode = GetBlockTemplateModes::Template;
    let rules = vec![GetBlockTemplateRules::SegWit];
    let capabilities = vec![];
    
    client.get_block_template(
        mode,
        &rules,
        &capabilities
    ).expect("Failed to get block template")
}
