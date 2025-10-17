use solana_sdk::{signature::Keypair, signer::Signer};

/// This program generates a new Solana-compatible ed25519 keypair
fn main() {
    // Generate a random ed25519 keypair (same curve Solana uses)
    let keypair = Keypair::new();

    // Public key in Base58 (standard Solana address format)
    let pubkey = keypair.pubkey().to_string();

    // Secret key bytes: 64 bytes = 32-byte secret + 32-byte public
    let secret_64 = keypair.to_bytes();

    // Display helpers
    let secret_base58 = bs58::encode(secret_64).into_string();
    let secret_json_array = serde_json::to_string(&secret_64.to_vec()).unwrap();

    println!("Public address (Base58): {pubkey}");
    println!("Secret key (Base58, 64 bytes): {secret_base58}");
    println!("Secret key (JSON array, 64 bytes): {secret_json_array}");
}
