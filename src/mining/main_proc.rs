use std::process::{Command, Child, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use bitcoin::Block;
use bitcoin::consensus::serialize;
use std::env;

// Increase number of workers later
const NUM_WORKERS: u32 = 1;
const NONCE_RANGE: u32 = u32::MAX / NUM_WORKERS;

// Remove the first argument
fn start_worker(_worker_id: u32, start_nonce: u32, end_nonce: u32, block_data: &[u8]) -> Child {
    Command::new(env::current_exe().unwrap()) // Use the current binary
        .arg("worker")
        .arg(start_nonce.to_string())
        .arg(end_nonce.to_string())
        .arg(hex::encode(block_data))
        .stdout(Stdio::piped()) // Capture output
        .spawn()
        .expect("Failed to start worker")
}

pub fn mine_main(block: Block) {
    let serialized_block = serialize(&block); // Serialize block for workers
    let mut workers: Vec<(Child, u32, u32, u32)> = vec![];

    for i in 0..NUM_WORKERS {
        let start = i * NONCE_RANGE;
        let end = start + NONCE_RANGE - 1;
        let worker = start_worker(i, start, end, &serialized_block);
        println!("Started worker {}: mining range {}-{}", i, start, end);
        workers.push((worker, i, start, end));
    }

    loop {
        for i in 0..workers.len() {
        // Add in this iterator
        // workers.into_iter().for_each(|worker| {

        // });
            // let (worker, worker_id, start, end) = &mut workers[i];
            let (worker, worker_id, _start, _end) = &mut workers[i];

            if let Ok(Some(status)) = worker.try_wait() {
                println!("Worker {} exited with status: {}", worker_id, status);
                // println!("Restarting worker {}...", worker_id);

                // Removing worker restart for now
                // let new_worker = start_worker(*worker_id, *start, *end, &serialized_block);
                // workers[i] = (new_worker, *worker_id, *start, *end);
            }

            // Read worker output asynchronously
            if let Some(ref mut stdout) = worker.stdout {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(output) = line {
                        if output.starts_with("FOUND") {
                            println!("\nBlock found! {}\n", output);
                            return; // Stop mining if a valid hash is found
                        }
                    }
                }
            }
        }

        thread::sleep(Duration::from_secs(2)); // Prevent 100% CPU usage, change to 1 later
    }
}
