# generate_keypair

Generate Solana ed25519 keypairs, create 12‑word mnemonics, or derive a Solana keypair from a mnemonic (no derivation path).

## Features

- Generate random Solana keypairs with Base58 and JSON array outputs.
- Create 12‑word English mnemonics and immediately show their derived Solana keypair.
- Derive a Solana keypair deterministically from a provided mnemonic, optionally with a passphrase.
- Clear CLI help and long-only flags to avoid ambiguity.

## Install

- Requirements: Rust/Cargo (stable)
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

- Generate 12‑word mnemonic(s) and show derived keypair(s):
  - cargo run -- --mnemonic
  - cargo run -- --mnemonic -n 3

- Derive from an existing mnemonic (no derivation path):
  - cargo run -- --mnemonic "abandon ... about"
  - cargo run -- --mnemonic "abandon ... about" -n 5
  - cargo run -- --mnemonic "..." --passphrase "my pass"

Notes:

- Use --mnemonic (two dashes). The single-dash -mnemonic is not supported and will be parsed as grouped short flags.
- Solana keypair derivation uses the Solana SDK’s built-in seed‑phrase function (no derivation path).
- Generated mnemonics default to 12 words using tiny‑bip39’s MnemonicType::Words12.

## Output format

Each keypair block prints:

- Header in yellow: “=== Keypair i ===”
- If generated via --mnemonic (no value), the 12‑word mnemonic.
- Public address in blue (Base58).
- 32‑byte public key as a JSON array.
- 64‑byte secret key in Base58 and as a JSON array (ed25519 sk||pk).

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

## License

MIT
