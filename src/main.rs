use clap::{Parser, Subcommand};
use colored::*;
use serde::Serialize;
use solana_sdk::signature::{Keypair, Signer};
use bip39::{Language, Mnemonic, Seed};

#[derive(Parser, Debug)]
#[command(name = "generate_keypair", version, about = "Generate Solana ed25519 keypairs")]
struct Cli {
    #[command(subcommand)]
    command: Option<Cmd>,
    /// Number of random keypairs (default 1) for default mode
    #[arg(short = 'n', long = "num", default_value_t = 1)]
    num: usize,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Generate a single keypair from a BIP39 mnemonic (no derivation path)
    Mnemonic {
        /// BIP39 mnemonic phrase
        #[arg(long)]
        phrase: String,
        /// Optional BIP39 passphrase
        #[arg(long, default_value = "")]
        passphrase: String,
    },
}

#[derive(Serialize)]
struct KeypairJson {
    public_key_base58: String,
    public_key_array_32: Vec<u8>,
    secret_key_base58_64: String,
    secret_key_array_64: Vec<u8>,
}

fn print_keypair(kp: &Keypair, index: Option<usize>) {
    let pubkey = kp.pubkey();
    let secret = kp.to_bytes(); // 64 bytes (sk||pk)
    let pub_bytes = pubkey.to_bytes(); // 32 bytes

    if let Some(i) = index {
        println!("=== Keypair {} ===", i + 1);
    } else {
        println!("=== Keypair ===");
    }

    println!("Public address (Base58): {}", pubkey.to_string().blue());
    println!(
        "Public key (JSON array, 32 bytes): {}",
        serde_json::to_string(&pub_bytes.to_vec()).unwrap()
    );
    println!(
        "Secret key (Base58, 64 bytes): {}",
        bs58::encode(&secret).into_string().purple()
    );
    println!(
        "Secret key (JSON array, 64 bytes): {}",
        serde_json::to_string(&secret.to_vec()).unwrap()
    );
    println!();
}

fn keypair_from_mnemonic(phrase: &str, passphrase: &str) -> Result<Keypair, String> {
    let mnemonic = Mnemonic::parse_in(Language::English, phrase)
        .map_err(|e| format!("mnemonic error: {e}"))?;
    let seed = Seed::new(&mnemonic, passphrase);
    let seed_bytes = seed.as_bytes();
    if seed_bytes.len() < 32 {
        return Err("seed bytes < 32".into());
    }
    // ASK/no-derivation-path mode: use first 32 bytes as ed25519 seed
    let mut seed32 = [0u8; 32];
    seed32.copy_from_slice(&seed_bytes[..32]);
    // Build a Solana Keypair deterministically from the 32-byte seed
    let kp = Keypair::from_seed(&seed32)
        .map_err(|e| format!("keypair from seed error: {e}"))?;
    Ok(kp)
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Cmd::Mnemonic { phrase, passphrase }) => {
            match keypair_from_mnemonic(phrase, passphrase) {
                Ok(kp) => print_keypair(&kp, None),
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        None => {
            for i in 0..cli.num {
                let kp = Keypair::new();
                print_keypair(&kp, Some(i));
            }
        }
    }
}
