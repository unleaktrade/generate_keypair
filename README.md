# 🔑 generate_keypair

A small but powerful **Rust CLI tool** that generates one or more **Solana-compatible ed25519 keypairs**, printing both the public and secret keys in an easy-to-read, colorized format.  
The public key (Solana address) is displayed in **blue**, while the secret key is shown in **purple**, following Solana’s standard Base58 encoding.  
This project is perfect for developers, blockchain engineers, or anyone experimenting with Solana cryptography and key management directly in Rust.

---

## 🚀 Overview

This utility provides a simple and transparent way to generate valid Solana keypairs without relying on precompiled Solana CLI binaries (`solana-keygen`).  
It uses the same cryptographic primitives as Solana itself via the `solana-sdk` crate, ensuring that keys are fully compatible with the Solana blockchain ecosystem.  

Each run produces **random ed25519 keypairs**, which you can safely import into wallets or use for signing and development tasks.  
You can also specify the number of keypairs to generate using a command-line argument, allowing quick bulk generation for testnets or automated setups.

---

## ✨ Features

- 🔐 **Generates authentic Solana keypairs** using `solana-sdk`  
- 🧮 **Supports multiple key generation** via CLI argument (e.g., `cargo run -- 50`)
- 🟦 **Colorized output** for readability (blue = public, purple = secret)
- 💾 **Shows both Base58 and JSON array formats** for secret keys  
- 🧰 **Lightweight and dependency-minimal** (no external Solana tools required)
- ⚙️ **Built with Rust 2024 edition**, ready for modern builds and toolchains

---

## ⚙️ Installation

You’ll need [Rust](https://www.rust-lang.org/tools/install) and Cargo installed.  
Clone this repository and build the project with:

```bash
git clone https://github.com/yourname/generate_keypair.git
cd generate_keypair
cargo build --release
```

After building, you can find the compiled binary at:

```bash
target/release/generate_keypair
```

---

## 💡 Usage

### Generate a single keypair

```bash
cargo run
```

This will generate one random keypair and print the public and secret keys with color formatting.

### Generate multiple keypairs

To generate multiple keypairs, use a numeric argument after `--`:

```bash
cargo run -- 50
```

This creates 50 new Solana-compatible keypairs in a single run.  
Each keypair is displayed in a clean, separated section for easy reading or copying.

---

## 🧱 Example Output

```bash
=== Keypair 1 ===
Public address (Base58):  9xQeWvG816bUx9EP8YkH...
Public key (JSON array, 32 bytes): [4,128,85,231,146,173,...]
Secret key (Base58, 64 bytes):  3cYgJz1u8qY9...
Secret key (JSON array, 64 bytes):  [154,232,43,102,21,...]

=== Keypair 2 ===
Public address (Base58):  2d8nAs5pUz3Bt4Uv...
Public key (JSON array, 32 bytes): [228,190,161,153,97,224,...]
Secret key (Base58, 64 bytes):  4DYqKxE9PfDq...
Secret key (JSON array, 64 bytes):  [205,178,54,92,...]
```

Each public key corresponds to a Solana address and can be imported directly into wallets or tools that support standard Solana keypair formats.

---

## 📦 Dependencies

| Crate | Purpose |
|-------|----------|
| [`solana-sdk`](https://docs.rs/solana-sdk/latest/solana_sdk/) | Core Solana types and ed25519 key handling |
| [`bs58`](https://crates.io/crates/bs58) | Base58 encoding for Solana public/secret keys |
| [`serde_json`](https://crates.io/crates/serde_json) | JSON serialization for secret key arrays |
| [`colored`](https://crates.io/crates/colored) | Terminal color and bold text formatting |

All dependencies are stable and well-maintained, with no unsafe code paths or external system dependencies.

---

## 🛡️ Security Notes

- This tool **prints secret keys to stdout**, which is not secure for production use.  
  Only use it for local development, testing, or when generating keys for temporary purposes.
- Avoid redirecting output to files unless you plan to **securely store** or **encrypt** them afterward.
- For production-grade key management, prefer:
  - The official [`solana-keygen`](https://docs.solana.com/cli/usage#solana-keygen)
  - A hardware wallet
  - An encrypted keystore or secret management system

Remember: **Anyone with access to your secret key can control your funds.**

---

## 🧭 Roadmap

Future improvements planned:

- [ ] Optional output to JSON or YAML file  
- [ ] Integration with environment variable secrets (`.env`)  
- [ ] Deterministic seed-based generation  
- [ ] CLI flags (`--json`, `--out`, `--color=false`)  
- [ ] Support for exporting `.json` wallet files compatible with Phantom/Solana CLI  

---

## 🧑‍💻 Development

You can easily modify the code to:

- Export keys to a file
- Return only public keys for whitelisting
- Integrate into larger Solana automation scripts

To format and lint the project:

```bash
cargo fmt
cargo clippy
```

To run tests (if you add any):

```bash
cargo test
```

---

## 🪪 License

MIT License © 2025 [Your Name]  
Free to use, modify, and distribute. Contributions welcome!

---

### 🌐 Links

- [Rust Documentation](https://doc.rust-lang.org/)
- [Solana Developer Docs](https://docs.solana.com/developing/programming-model/overview)
- [Crates.io Page](https://crates.io/)
- [Solana SDK on Docs.rs](https://docs.rs/solana-sdk)

---
