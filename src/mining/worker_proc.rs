use std::env;
use bitcoin::hashes::{sha256d, Hash};

// Update this to return the block itself (Maybe a Result or Option type)
pub fn mine_worker() -> Option<(String, u32, sha256d::Hash)> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        dbg!(&args);
        eprintln!("Usage: <executable_path> worker <start_nonce> <end_nonce> <block_data>");
        return None;
    }

    // args[0] is the path, args[1] is the string "worker"
    let start_nonce: u32 = args[2].parse().unwrap();
    let end_nonce: u32 = args[3].parse().unwrap();
    let block_data: String = args[4].parse().unwrap();
    // use the target provided by the template block
    let target = u32::MAX / 1000; // Simplified difficulty target

    println!("Worker mining from nonce {} to {}", start_nonce, end_nonce);

    for nonce in start_nonce..=end_nonce {
        // Update block's nonce
        let mining_data = block_data.clone();
        let mut mining_data_bytes = hex::decode(&mining_data).unwrap();
        mining_data_bytes.splice(76..80, nonce.to_le_bytes().iter().cloned());

        let hash = sha256d::Hash::hash(&mining_data_bytes);
        let hash_int = u32::from_le_bytes(hash[..4].try_into().unwrap());

        if hash_int < target {
            println!("FOUND {} {}", nonce, hash);
            return Some((mining_data, nonce, hash));
        }
    }

    println!("Worker {}-{} finished without finding a block.", start_nonce, end_nonce);
    None
}
