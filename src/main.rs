use clap::Parser;
use serde::{Deserialize, Serialize};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    name: String,
    count: u8,
}

fn main() {
    let args = Args::parse();

    let settings = Settings {
        name: args.name.clone(),
        count: args.count,
    };

    let serialised = serde_json::to_string(&settings).unwrap();
    println!("Serialized settings: {}", serialised);

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}