use std::env;
use bitcoin::Block;
use bitcoin::block::Header;

pub fn mine_worker() -> Option<Block> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        dbg!(&args);
        eprintln!("Usage: <executable_path> worker <start_nonce> <end_nonce> <block_data> <header>");
        return None;
    }

    // args[0] is the path, args[1] is the string "worker"
    let start_nonce: u32 = args[2].parse().unwrap();
    let end_nonce: u32 = args[3].parse().unwrap();
    let block_data: String = args[4].parse().unwrap();
    let header: String = args[5].parse().unwrap();

    println!("Worker mining from nonce {} to {}", start_nonce, end_nonce);

    // Get the transaction data from the original block
    let block_bytes = hex::decode(&block_data).unwrap();
    let tx_bytes = block_bytes[80..].to_vec();

    for nonce in start_nonce..=end_nonce {
        // Update block's nonce
        let header_str = header.clone();
        let mut header_bytes = hex::decode(&header_str).unwrap();
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
