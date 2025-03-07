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
