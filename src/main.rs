use colored::*;
use solana_sdk::{signature::Keypair, signer::Signer};
use std::env;
use std::str::FromStr;

fn main() {
    // Parse count from first CLI arg (default: 1)
    let args: Vec<String> = env::args().skip(1).collect();
    let count = args
        .get(0)
        .map(|s| usize::from_str(s).unwrap_or_else(|_| fail(s)))
        .unwrap_or(1);

    for i in 1..=count {
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey().to_string();
        let secret_64 = keypair.to_bytes();
        let secret_base58 = bs58::encode(secret_64).into_string();
        let secret_json_array = serde_json::to_string(&secret_64.to_vec()).unwrap();

        println!("{}", format!("=== Keypair {i} ===").bold().bright_yellow());
        println!(
            "{} {}",
            "Public address (Base58):".bold(),
            pubkey.bold().bright_cyan()
        );
        println!(
            "{} {}",
            "Private key (Base58, 64 bytes):".bold(),
            secret_base58.bright_purple()
        );
        println!(
            "{} {}",
            "Private key (JSON array, 64 bytes):".bold(),
            secret_json_array
        );
        if i != count {
            println!(); // blank line between entries
        }
    }
}

fn fail(got: &str) -> ! {
    eprintln!("{} {}", "Invalid count:".bold().red(), got);
    eprintln!("Usage: cargo run -- <count>");
    std::process::exit(2);
}
