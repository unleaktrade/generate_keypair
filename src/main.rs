use colored::*;
use solana_sdk::{signature::Keypair, signer::Signer};

fn main() {
    // Generate a random ed25519 keypair (Solana-compatible)
    let keypair = Keypair::new();

    // Public key (Base58) and secret key (64-byte combined)
    let pubkey = keypair.pubkey().to_string();
    let secret_64 = keypair.to_bytes();
    let secret_base58 = bs58::encode(secret_64).into_string();
    let secret_json_array = serde_json::to_string(&secret_64.to_vec()).unwrap();

    // Pretty, colored output
    println!(
        "{} {}",
        "Public address (Base58):".bold(),
        pubkey.bold().bright_blue()
    );
    println!(
        "{} {}",
        "Private key (Base58, 64 bytes):".bold(),
        secret_base58.bold().bright_purple()
    );
    println!(
        "{} {}",
        "Private key (JSON array, 64 bytes):".bold(),
        secret_json_array
    );
}
