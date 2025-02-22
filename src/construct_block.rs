use bitcoincore_rpc::json::GetBlockTemplateResult;
use bitcoin::{block::{Block, Header, Version}, CompactTarget, Transaction};
use crate::calculate_merkle_root::calculate_merkle_root;
use crate::build_coinbase::create_coinbase_transaction;
use bitcoin::consensus::Decodable;
use bitcoin::Txid;

pub fn construct_block(template: GetBlockTemplateResult, miner_address: &str) -> Block {
    // Create coinbase transaction first
    let coinbase_tx = create_coinbase_transaction(
        template.height as u32,
        template.coinbase_value.to_sat(),
        miner_address
    );

    let version = Version::from_consensus(template.version.try_into().unwrap());
    let current_time = template.current_time as u32;
    let bits = CompactTarget::from_hex(&format!("0x{}", hex::encode(template.bits))).unwrap();

    // Convert template transactions to Bitcoin transactions
    let mut transactions: Vec<Transaction> = template.transactions
    .iter()
    .map(|tx| {
        let tx_bytes = hex::decode(&tx.raw_tx).unwrap();
        Transaction::consensus_decode(&mut tx_bytes.as_slice()).unwrap()
    })
    .collect();

    // Insert coinbase as first transaction
    transactions.insert(0, coinbase_tx);

    // Create block header
    let header = Header {
        version,
        prev_blockhash: template.previous_block_hash,
        merkle_root: calculate_merkle_root(&transactions.iter().map(|tx| tx.compute_txid()).collect::<Vec<Txid>>()),
        time: current_time,
        bits,
        nonce: 0 // Initial nonce value for mining
    };

    // Construct full block with transactions
    Block {
        header,
        txdata: transactions
    }
}
