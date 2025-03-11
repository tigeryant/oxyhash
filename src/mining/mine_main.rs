use bitcoin::Block;
use super::mine_worker::mine_worker;

const NONCE_RANGE: u32 = u32::MAX;

pub fn mine_main(block: Block) -> Block {
    let start = 0;
    let end = start + NONCE_RANGE - 1;
    println!("Started mining range {}-{}", start, end);
    mine_worker(start, end, block).unwrap()
}
