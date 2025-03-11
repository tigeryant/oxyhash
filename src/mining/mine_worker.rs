use bitcoin::Block;
use bitcoin::block::Header;

pub fn mine_worker(start_nonce: u32, end_nonce: u32, block_data: &[u8], header: Header) -> Option<Block> {
    println!("Worker mining from nonce {} to {}", start_nonce, end_nonce);

    // Get the transaction data from the original block
    let block_bytes = block_data;
    let tx_bytes = block_bytes[80..].to_vec();

    for nonce in start_nonce..=end_nonce {
        // Update block's nonce
        let mut header_bytes = bitcoin::consensus::serialize(&header);
        header_bytes.splice(76..80, nonce.to_le_bytes().iter().cloned());

        let block_header: Header = bitcoin::consensus::deserialize(&header_bytes).unwrap();
        let block_hash = block_header.block_hash();
        let target = block_header.target();

        let header_bytes = bitcoin::consensus::serialize(&block_header);

        // Combine header and transaction bytes
        let mut complete_block_bytes = header_bytes;
        complete_block_bytes.extend_from_slice(&tx_bytes);

        // Deserialize into a Block
        let valid_block: Block = bitcoin::consensus::deserialize(&complete_block_bytes).unwrap();

        if target.is_met_by(block_hash) {
            println!("FOUND {} {}", nonce, block_hash);
            return Some(valid_block);
        }
    }

    println!("Worker {}-{} finished without finding a block.", start_nonce, end_nonce);
    None
}
