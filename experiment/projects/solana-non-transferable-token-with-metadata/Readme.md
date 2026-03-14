# Solana Non-Transferable Token with Metadata

A Rust application that creates **Non-Transferable SPL Tokens** with on-chain metadata on the Solana blockchain using the **Token-2022** program. Perfect for soulbound tokens, achievement badges, credentials, and membership proofs that should not be traded or transferred.

---

## Table of Contents

- [What is This?](#what-is-this)
- [Key Concepts](#key-concepts)
- [Prerequisites](#prerequisites)
- [Setup](#setup)
- [Usage](#usage)
- [How It Works](#how-it-works)
- [Project Structure](#project-structure)
- [Output](#output)
- [References](#references)

---

## What is This?

This project demonstrates how to mint a **Non-Transferable Token** on Solana with:

- **Token-2022** (SPL Token 2022) — the next-generation token program with extensible features
- **Non-Transferable extension** — tokens cannot be sent between wallets (soulbound)
- **Metadata extension** — name, symbol, URI, and custom fields stored on-chain
- **Metadata Pointer extension** — links the mint to its metadata account

Use cases include:

- Achievement badges and certifications  
- Membership or identity credentials  
- Event attendance proofs  
- Loyalty or reputation points that shouldn’t be traded  

---

## Key Concepts

| Term | Description |
|------|-------------|
| **SPL Token** | Solana’s standard for fungible and non-fungible tokens |
| **Token-2022** | Upgraded token program with extensions (metadata, non-transferable, etc.) |
| **Non-Transferable** | Tokens that cannot be transferred; they stay in the original wallet |
| **Metadata** | On-chain data: name, symbol, URI, and custom key-value pairs |
| **Mint** | The account that defines a token type (supply, decimals, extensions) |
| **ATA (Associated Token Account)** | A wallet’s token account for a specific mint |

---

## Prerequisites

- **Rust** (1.70+): [rustup.rs](https://rustup.rs)
- **Solana CLI** (optional, for keygen): [Solana docs](https://docs.solana.com/cli/install-solana-cli-tools)
- **Devnet SOL** for transaction fees: [Solana Faucet](https://faucet.solana.com)

---

## Setup

### 1. Clone and build

```bash
cd solana-non-transferable-token-with-metadata
cargo build
```

### 2. Create a payer keypair (if needed)

```bash
solana-keygen new
```

### 3. Configure environment

Create a `.env` file in the project root:

```env
PAYER_PRIVATE_KEY=<your-base58-private-key>
```

You can get the base58 private key from your keypair file or by exporting:

```bash
solana-keygen pubkey ~/.config/solana/id.json  # get public key
# For private key, use the array from the JSON file or export as base58
```

### 4. Fund your wallet on Devnet

```bash
solana airdrop 2 --url devnet
```

---

## Usage

Run the program:

```bash
cargo run
```

On success, you’ll see:

- Payer public key  
- Mint address  
- Transaction signature  
- Enabled extensions  
- Metadata pointer and token metadata details  

---

## How It Works

The program builds a single transaction with these steps:

1. **Create mint account** — Allocate space for the mint with extensions.
2. **Initialize Non-Transferable** — Enable the extension that blocks transfers.
3. **Initialize Metadata Pointer** — Point the mint to its metadata (self-referencing).
4. **Initialize mint** — Set decimals, mint authority, freeze authority.
5. **Initialize token metadata** — Set name, symbol, and URI.
6. **Update fields** — Add custom metadata (e.g. `description`).
7. **Create ATA** — Create an associated token account for the payer and mint tokens.

Flow:

```
Payer Keypair → Create Mint → Add Extensions → Set Metadata → Create ATA
```

---

## Project Structure

```
solana-non-transferable-token-with-metadata/
├── Cargo.toml          # Dependencies (Solana SDK, SPL Token 2022, etc.)
├── src/
│   └── main.rs         # Main logic: mint creation, extensions, metadata
├── .env                # PAYER_PRIVATE_KEY (not committed)
└── Readme.md           # This file
```

### Dependencies

| Crate | Purpose |
|-------|---------|
| `solana-client` | RPC client for Solana |
| `solana-sdk` | Core types (keypair, transaction) |
| `spl-token-2022-interface` | Token-2022 instructions and extensions |
| `spl-token-metadata-interface` | Metadata extension |
| `spl-associated-token-account-interface` | ATA creation |
| `dotenvy` | Load `.env` variables |
| `anyhow` | Error handling |
| `tokio` | Async runtime |

---

## Output

Example output:

```
payer = HiMmuCbieNgDNFd9GbcbVSHYPGPuEgZWwQxJULaJVoVs
Mint Address: 98f3PsJWBfgbA9pbTxfVyjiSfNYancM7yutHqkkwdQLG
Transaction Signature: 52aqH9z6dhxhqCpyGKPNJ6inAXhGrHxLSUecqeHH9j3tjvzWUQ1p4ML2La9HNBUsG6QDBMkCgUa3Z7cHrHStui8y

Extensions enabled: [NonTransferable, MetadataPointer, TokenMetadata]

MetadataPointer { ... }
TokenMetadata { name: "FirstDayAtArena", symbol: "FDAA", uri: "...", ... }
```

View the mint on Solana Explorer (Devnet):

```
https://explorer.solana.com/address/<MINT_ADDRESS>?cluster=devnet
```

---

## References

- [Token-2022 Program](https://spl.solana.com/token-2022)
- [Token Extensions](https://solana.com/developers/guides/token-extensions/getting-started)
- [Non-Transferable Tokens](https://solana.com/developers/guides/token-extensions/non-transferable-tokens)
- [Metadata Extension](https://solana.com/developers/guides/token-extensions/metadata-pointer)
