# generate_keypair

Generate Solana ed25519 keypairs, create 12/15/18/21/24‑word mnemonics, or derive a Solana keypair from a mnemonic (no derivation path).

## Features

- Generate random Solana keypairs with Base58 and JSON array outputs.
- Create 12, 15, 18, 21 or 24‑word English mnemonics (`--words`, default 12) and immediately show their derived Solana keypair.
- Derive a Solana keypair deterministically from a provided mnemonic (validated as English BIP39 first).
- Optional BIP39 passphrase, applied both when generating a mnemonic and when deriving from a provided one.
- Clear CLI help; every option has a long and a short form (`-n/--num`, `-m/--mnemonic`, `-p/--passphrase`, `-w/--words`), plus `-h/--help` and `-V/--version`.

## Install

- Requirements: Rust/Cargo 1.85 or newer (the crate uses edition 2024)
- Build:
  - git clone <https://github.com/unleaktrade/generate_keypair>
  - cd generate_keypair
  - cargo build --release

## Usage

- Help/usage:
  - cargo run -- --help

- Random keypairs (default mode):
  - cargo run
  - cargo run -- -n 10

- Generate mnemonic(s) and show derived keypair(s) (12 words by default):
  - cargo run -- --mnemonic
  - cargo run -- --mnemonic -n 3
  - cargo run -- --mnemonic --words 24
  - cargo run -- --mnemonic --passphrase "my pass"

- Derive from an existing mnemonic (no derivation path):
  - cargo run -- --mnemonic "abandon ... about"
  - cargo run -- --mnemonic "abandon ... about" -n 5 (prints the same keypair 5 times: there is no derivation index)
  - cargo run -- --mnemonic "..." --passphrase "my pass"

Notes:

- Use --mnemonic or -m. A single-dash -mnemonic is read as `-m nemonic` and fails with an invalid‑mnemonic error.
- `--words` accepts 12, 15, 18, 21 or 24 and only applies when generating; it is ignored when a phrase is provided.
- `--passphrase` changes the derived keypair in both mnemonic modes: the same mnemonic with a different passphrase gives a different address.
- Solana keypair derivation uses the Solana SDK’s built-in seed‑phrase function (no derivation path). The resulting address therefore differs from wallets that derive with a path such as `m/44'/501'/0'/0'` (Phantom, or `solana-keygen` with a derivation path).
- Generated mnemonics default to 12 words using tiny‑bip39’s MnemonicType::Words12.

## Output format

Each keypair block prints:

- Header in yellow: “=== Keypair i ===”
- If generated via --mnemonic (no value), the mnemonic, labelled with its word count.
- Public address in blue (Base58).
- 32‑byte public key as a JSON array.
- 64‑byte secret key in Base58 (in purple) and as a JSON array (ed25519 sk||pk).

Example:

```bash
=== Keypair 1 ===
Mnemonic (12 words): people coast soap frown behind inspire minute sausage toss crucial brain ramp
Public address (Base58): HLynTUNfvYYQ...
Public key (JSON array, 32 bytes): [242,217,59,84,221,126,...]
Secret key (Base58, 64 bytes): Bdgg5SYksSYg29P...
Secret key (JSON array, 64 bytes): [9,43,70,219,255,108,...]
```

## 🧭 Roadmap

- [ ] Optional output to JSON or YAML file
- [ ] Environment‑variable integration for secrets
- [ ] Deterministic seed‑based generation variants
- [ ] CLI flags: --json, --out, --color=false
- [ ] Export .json wallet files compatible with Phantom/Solana CLI

## Development

- Format/lint: cargo fmt && cargo clippy
- Run: cargo run (see Usage)
- There are no automated tests yet.
