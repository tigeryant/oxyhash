use clap::{Parser, Subcommand};

/// A simple CLI application
#[derive(Parser)]
#[command(name = "mycli")]
#[command(version = "1.0")]
#[command(about = "OxyHash - multiprocess Bitcoin miner", long_about = None)]
pub struct Cli {
    /// Activate debug mode
    #[arg(short, long)]
    pub debug: bool,

    /// Optional input file
    #[arg(short, long)]
    pub input: Option<String>,

    /// Subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Address {
        address: String
    }
}

pub fn cli_args() -> (String, String) {
    let cli = Cli::parse();

    if cli.debug {
        println!("Debug mode enabled");
    }

    if let Some(input) = cli.input {
        println!("Input file: {}", input);
    }

    let miner_address: String = match cli.command {
        Some(Commands::Address { address }) => {
            // define the address
            println!("address defined");
            address
        }
        None => {
            println!("Default - using hardcoded miner address");
            String::from("bc1qq0hyc6ftal99hks3uspapyl8vcscqjf4aad7sp")
        },
    };

    // Update this to be passed as a CLI arg
    let socket_path = String::from("/Users/john/Development/bitcoin/bitcoin-node.sock");

    (miner_address, socket_path)
}
