use bitcoin::{Transaction, TxIn, TxOut, Script, Amount};
use bitcoin::blockdata::transaction::{OutPoint, Version};
use bitcoin::Network;
use bitcoin::Address;
use bitcoin::absolute::LockTime;
use std::str::FromStr;

pub fn create_coinbase_transaction(
    height: u32,
    reward: u64,
    miner_address: &str,
) -> Transaction {
    let coinbase_script = Script::builder()
    .push_int(height as i64)  // BIP-34: Block height in scriptsig
    .push_slice(b"OxyHash")   // Add mining pool identifier
    .into_script();

    let coinbase_input = TxIn {
        previous_output: OutPoint::null(),
        script_sig: coinbase_script,
        sequence: bitcoin::Sequence(0xffffffff),
        witness: Default::default(),
    };

    // Create the miner's output
    let miner_addr = Address::from_str(miner_address)
        .expect("Invalid miner address")
        .require_network(Network::Bitcoin)
        .expect("Wrong network");
        
    let miner_output = TxOut {
        value: Amount::from_sat(reward),
        script_pubkey: miner_addr.script_pubkey(),
    };

    Transaction {
        version: Version(1),
        lock_time: LockTime::ZERO,
        input: vec![coinbase_input],
        output: vec![miner_output],
    }
}
