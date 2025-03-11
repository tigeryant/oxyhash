use bitcoin::Block;
use bitcoin::consensus::serialize;
use super::mine_worker::mine_worker;

const NONCE_RANGE: u32 = u32::MAX;

pub fn mine_main(block: Block) -> Block {
    let serialized_block = serialize(&block); // Serialize block for workers
    let header = block.header;

    // pass the start, end, serialized block and header (or derive the header from the first 80 bytes)
    let start = 0;
    let end = start + NONCE_RANGE - 1;
    println!("Started mining range {}-{}", start, end);
    mine_worker(start, end, &serialized_block, header).unwrap()
}
