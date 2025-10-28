use anyhow::{Result, anyhow, bail};
use bip39::{Language, Mnemonic, MnemonicType}; // tiny-bip39 via Cargo.toml alias
use clap::Parser;
use colored::*;
use serde_json;
use solana_sdk::signature::{Keypair, SeedDerivable, Signer};

#[derive(Parser, Debug)]
#[command(
    name = "generate_keypair",
    version,
    about = "Generate Solana ed25519 keypairs; generate 12/15/18/21/24-word mnemonics; or derive from a mnemonic (no derivation path).",
    long_about = None,
    disable_help_subcommand = true
)]
struct Cli {
    /// Count:
    /// - Random mode: number of keypairs
    /// - Mnemonic generation: number of mnemonic+keypair pairs
    /// - Derive-from-mnemonic: repeats the same keypair N times (no path)
    #[arg(short = 'n', long = "num", default_value_t = 1)]
    num: usize,

    /// Mnemonic mode (long-only):
    /// - If set without a value: generate English mnemonic(s) and show derived keypair(s)
    /// - If set with a phrase: derive Solana keypair(s) from that phrase (no derivation path)
    ///
    /// Examples:
    ///   --mnemonic
    ///   --mnemonic -n 5
    ///   --mnemonic "abandon abandon ... about"
    #[arg(short = 'm', long = "mnemonic")]
    mnemonic: Option<Option<String>>,

    /// Optional BIP39 passphrase when deriving from a provided phrase
    #[arg(short = 'p', long = "passphrase", default_value = "")]
    passphrase: String,

    /// Word length for generated mnemonics (generation mode only). Ignored when a phrase is provided.
    /// Allowed: 12, 15, 18, 21, 24. Default: 12.
    #[arg(short = 'w', long = "words", default_value_t = 12)]
    words: usize,
}

fn print_full_block(kp: &Keypair, mnemonic_for_display: Option<&str>, index: usize) {
    let pubkey = kp.pubkey();
    let secret = kp.to_bytes(); // 64 bytes (sk||pk)
    let pub_bytes = pubkey.to_bytes(); // 32 bytes

    println!("{}", format!("=== Keypair {} ===", index).yellow());
    if let Some(m) = mnemonic_for_display {
        println!("Mnemonic ({} words): {}", m.split_whitespace().count(), m);
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

fn mnemonic_type_from_len(words: usize) -> Result<MnemonicType> {
    let mt = match words {
        12 => MnemonicType::Words12,
        15 => MnemonicType::Words15,
        18 => MnemonicType::Words18,
        21 => MnemonicType::Words21,
        24 => MnemonicType::Words24,
        _ => bail!("invalid --words value; allowed: 12, 15, 18, 21, 24"),
    };
    Ok(mt)
}

fn gen_mnemonic(words: usize) -> Result<String> {
    let mt = mnemonic_type_from_len(words)?;
    let m = Mnemonic::new(mt, Language::English);
    Ok(m.phrase().to_string())
}

fn keypair_from_mnemonic(phrase: &str, passphrase: &str) -> Result<Keypair> {
    // Validate phrase (tiny-bip39) then derive Solana ed25519 keypair (no path)
    Mnemonic::from_phrase(phrase, Language::English)
        .map_err(|e| anyhow!("invalid BIP39 mnemonic: {e}"))?;
    let kp = Keypair::from_seed_phrase_and_passphrase(phrase, passphrase)
        .map_err(|e| anyhow!("failed to derive keypair: {e}"))?;
    Ok(kp)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.mnemonic {
        // Generation mode: create mnemonic(s) with chosen word length, derive keypair(s), print both
        Some(None) => {
            for i in 1..=cli.num {
                let phrase = gen_mnemonic(cli.words)?;
                let kp = keypair_from_mnemonic(&phrase, &cli.passphrase)?;
                print_full_block(&kp, Some(&phrase), i);
            }
        }
        // Derivation mode: use provided phrase, ignore --words
        Some(Some(phrase)) => {
            let kp = keypair_from_mnemonic(phrase, &cli.passphrase)?;
            for i in 1..=cli.num {
                print_full_block(&kp, None, i);
            }
        }
        // Default random mode
        None => {
            for i in 1..=cli.num {
                let kp = Keypair::new();
                print_full_block(&kp, None, i);
            }
        }
    }

    Ok(())
}
