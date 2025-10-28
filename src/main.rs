use anyhow::Result;
use bip39::{Language, Mnemonic, MnemonicType}; // tiny-bip39
use clap::Parser;
use colored::*;
use serde_json;
use solana_sdk::signature::{Keypair, SeedDerivable, Signer};

#[derive(Parser, Debug)]
#[command(
    name = "generate_keypair",
    version,
    about = "Generate Solana ed25519 keypairs (random), generate mnemonics, or derive from mnemonic"
)]
struct Cli {
    /// Count:
    /// - Random mode: number of keypairs
    /// - Mnemonic generation: number of mnemonics
    /// - Derive-from-mnemonic: repeats the same keypair N times (no path)
    #[arg(short = 'n', long = "num", default_value_t = 1)]
    num: usize,

    /// Mnemonic mode (long-only):
    /// - If set without a value: generate 12-word English mnemonics
    /// - If set with a phrase: derive Solana keypair(s) from that phrase (no derivation path)
    ///
    /// Examples:
    ///   --mnemonic
    ///   --mnemonic -n 5
    ///   --mnemonic "abandon abandon ... about"
    #[arg(long = "mnemonic")]
    mnemonic: Option<Option<String>>,

    /// Optional BIP39 passphrase used only when deriving from a provided phrase
    #[arg(long = "passphrase", default_value = "")]
    passphrase: String,
}

fn print_keypair(kp: &Keypair, index: Option<usize>) {
    let pubkey = kp.pubkey();
    let secret = kp.to_bytes();        // 64 bytes (sk||pk)
    let pub_bytes = pubkey.to_bytes(); // 32 bytes

    let header = match index {
        Some(i) => format!("=== Keypair {} ===", i + 1),
        None => "=== Keypair ===".to_string(),
    };
    println!("{}", header.yellow());
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

fn gen_mnemonic_12() -> String {
    let m = Mnemonic::new(MnemonicType::Words12, Language::English);
    m.phrase().to_string()
}

fn keypair_from_mnemonic(phrase: &str, passphrase: &str) -> Result<Keypair> {
    // Validate phrase (tiny-bip39) and derive via Solana SDK (no path)
    Mnemonic::from_phrase(phrase, Language::English)
        .map_err(|e| anyhow::anyhow!("invalid BIP39 mnemonic: {e}"))?;
    let kp = Keypair::from_seed_phrase_and_passphrase(phrase, passphrase)
        .map_err(|e| anyhow::anyhow!("failed to derive keypair: {e}"))?;
    Ok(kp)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.mnemonic {
        // --mnemonic with no value -> generate 12-word English mnemonics
        Some(None) => {
            for _ in 0..cli.num {
                let phrase = gen_mnemonic_12();
                println!("{}", "=== Mnemonic (12 words) ===".yellow());
                println!("{}", phrase);
                println!();
            }
        }
        // --mnemonic "phrase" -> derive keypair(s) from provided phrase
        Some(Some(phrase)) => {
            let kp = keypair_from_mnemonic(phrase, &cli.passphrase)?;
            for i in 0..cli.num {
                print_keypair(&kp, Some(i));
            }
        }
        // No --mnemonic -> random Solana keypairs
        None => {
            for i in 0..cli.num {
                let kp = Keypair::new();
                print_keypair(&kp, Some(i));
            }
        }
    }

    Ok(())
}
