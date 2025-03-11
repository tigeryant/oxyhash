use bitcoincore_rpc::json::GetBlockTemplateResult;
use bitcoincore_rpc::{Client, RpcApi};
use bitcoincore_rpc::bitcoincore_rpc_json::{
    GetBlockTemplateModes,
    GetBlockTemplateRules
};

// Update this to use the IPC interface (blocktalk)
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
