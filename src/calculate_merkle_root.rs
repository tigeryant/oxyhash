use bitcoin::hashes::{Hash, sha256d, HashEngine};
use bitcoin::Txid;

pub fn calculate_merkle_root(transactions: &[Txid]) -> bitcoin::hash_types::TxMerkleNode {
    if transactions.is_empty() {
        return bitcoin::hash_types::TxMerkleNode::all_zeros();
    }

    let mut current_level: Vec<[u8; 32]> = transactions
        .iter()
        .map(|txid| txid.to_byte_array())
        .collect();

    while current_level.len() > 1 {
        let mut next_level = Vec::new();
        
        for chunk in current_level.chunks(2) {
            let mut hasher = sha256d::Hash::engine();
            hasher.input(&chunk[0]);
            // If odd number of elements, duplicate the last one
            hasher.input(chunk.get(1).unwrap_or(&chunk[0]));
            
            next_level.push(sha256d::Hash::from_engine(hasher).to_byte_array());
        }
        
        current_level = next_level;
    }

    bitcoin::hash_types::TxMerkleNode::from_slice(&current_level[0]).unwrap()
}
