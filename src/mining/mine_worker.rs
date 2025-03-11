use bitcoin::Block;
use bitcoin::block::Header;
use bitcoin::consensus::{serialize, deserialize};

pub fn mine_worker(start_nonce: u32, end_nonce: u32, block_template: Block) -> Option<Block> {
    println!("Worker mining from nonce {} to {}", start_nonce, end_nonce);

    // Get the transaction data from the original block
    let header = block_template.header;
    let block_bytes = serialize(&block_template);
    let tx_bytes = &block_bytes[80..];

    for nonce in start_nonce..=end_nonce {
        let modified_header = update_header(header, nonce);

        let hash = modified_header.block_hash();

        let target = modified_header.target();

        if target.is_met_by(hash) {
            println!("FOUND {} {}", nonce, hash);
            let valid_block = construct_block(header, tx_bytes);
            return Some(valid_block);
        }
    }

    println!("Worker {}-{} finished without finding a block.", start_nonce, end_nonce);
    None
}

// Update the header with a new nonce
fn update_header(header: Header, nonce: u32) -> Header {
    let mut header_bytes = serialize(&header);
    header_bytes.splice(76..80, nonce.to_le_bytes().iter().cloned());
    deserialize(&header_bytes).unwrap()
}

// Concatenate the header bytes with the transaction bytes to form a new block
fn construct_block(header: Header, tx_bytes: &[u8]) -> Block {
    let header_bytes = serialize(&header);

    // Combine header and transaction bytes
    let mut complete_block_bytes = header_bytes;
    complete_block_bytes.extend_from_slice(tx_bytes);

    // Deserialize into a Block
    let valid_block: Block = deserialize(&complete_block_bytes).unwrap();
    valid_block
}
