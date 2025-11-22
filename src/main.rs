mod scanner;
mod utils;

use crate::scanner::network::scan_network;
use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ScanPorts {
        /// Target host IP address
        #[arg(short = 't', long, default_value = "127.0.0.1")]
        host: String,
    },
    ScanNetwork {
        /// Target network in CIDR notation
        #[arg(short, long)]
        network: String,
    },
}

// might need later

// #[derive(Serialize, Deserialize, Debug)]
// struct Settings {
//     ip_addr: String,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // let settings = Settings {
    //     ip_addr: cli.ip_addr.clone(),
    // };

    // let serialised = serde_json::to_string(&settings).unwrap();
    // println!("Serialized settings: {}", serialised);
    match &cli.command {
        Commands::ScanPorts { host } => {
            println!("Scanning ports on host: {}", host);
            scan_network(host);
        }
        Commands::ScanNetwork { network } => {
            println!("Scanning network: {}", network);
        }
    }

    Ok(())
}
