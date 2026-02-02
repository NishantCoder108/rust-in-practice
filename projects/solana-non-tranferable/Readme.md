# Solana Non-Transferable Token

A Rust application that demonstrates how to create **Non-Transferable SPL Tokens** on the Solana blockchain using the **Token-2022** program. This project is ideal for learning how soulbound tokens work—tokens that cannot be transferred between wallets once minted.

---

## Table of Contents

- [What is This?](#what-is-this)
- [Key Concepts](#key-concepts)
- [Prerequisites](#prerequisites)
- [Setup](#setup)
- [Usage](#usage)
- [How the Code Works](#how-the-code-works)
- [Project Structure](#project-structure)
- [Expected Output](#expected-output)
- [References](#references)

---

## What is This?

This project shows you how to:

1. **Create a Non-Transferable Token** — A token that cannot be sent to another wallet
2. **Use Token-2022** — Solana’s next-generation token program with extensions
3. **Verify the behavior** — The program intentionally attempts a transfer to prove it fails

**Common use cases:**

- Achievement badges and certifications  
- Membership or identity credentials  
- Event attendance proofs  
- Loyalty points that shouldn’t be traded  

---

## Key Concepts

| Term | Description |
|------|-------------|
| **SPL Token** | Solana’s standard for fungible and non-fungible tokens |
| **Token-2022** | Upgraded token program with extensions (non-transferable, metadata, etc.) |
| **Non-Transferable** | Extension that blocks all transfers; tokens stay in the original wallet |
| **Mint** | The account that defines a token type (supply, decimals, extensions) |
| **ATA (Associated Token Account)** | A wallet’s token account for a specific mint |
| **Fee Payer** | The wallet that pays transaction fees (rent, gas) |

---

## Prerequisites

- **Rust** (1.70+): [rustup.rs](https://rustup.rs)
- **Solana CLI**: [Solana docs](https://docs.solana.com/cli/install-solana-cli-tools)

---

## Setup

### 1. Start a local Solana validator

This project uses a **local validator** (not devnet). In a separate terminal:

```bash
solana-test-validator
```

Keep it running. The program connects to `http://localhost:8899`.

### 2. Build the project

```bash
cd solana-non-tranferable
cargo build
```

---

## Usage

Run the program:

```bash
cargo run
```

On success, you’ll see:

- Fee payer address  
- Mint address  
- Extensions enabled on the mint  
- Non-transferable extension data  
- A failed transfer attempt (expected behavior)  

---

## How the Code Works

The program runs in a single flow. Below is a step-by-step breakdown of what each part does.

### Phase 1: Connect and fund the fee payer

```rust
// Connect to local validator
let client = RpcClient::new_with_commitment(
    String::from("http://localhost:8899"),
    CommitmentConfig::confirmed(),
);

// Generate a new keypair (no pre-existing wallet needed)
let fee_payer = Keypair::new();

// Airdrop 5 SOL so we can pay for transactions
let airdrop_signature = client.request_airdrop(&fee_payer.pubkey(), 5_000_000_000).await?;
```

**What’s happening:** We create an RPC client, generate a new keypair, and airdrop SOL so we can pay for rent and transaction fees.

---

### Phase 2: Create the mint account with Non-Transferable extension

```rust
// Calculate how much space the mint needs (base mint + NonTransferable extension)
let mint_space = ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::NonTransferable])?;

// Create the account
let create_mint_account_instruction = create_account(
    &fee_payer.pubkey(),    // payer
    &mint.pubkey(),         // new mint account
    mint_rent,              // lamports for rent
    mint_space as u64,      // space in bytes
    &TOKEN_2022_PROGRAM_ID, // owner = Token-2022 program
);

// Enable the Non-Transferable extension
let initialize_non_transferable_instruction =
    initialize_non_transferable_mint(&TOKEN_2022_PROGRAM_ID, &mint.pubkey())?;

// Initialize mint data (decimals, authorities)
let initialize_mint_instruction = initialize_mint(
    &TOKEN_2022_PROGRAM_ID,
    &mint.pubkey(),
    &fee_payer.pubkey(),       // mint authority
    Some(&fee_payer.pubkey()), // freeze authority
    0,                         // decimals (0 is common for badges/certificates)
)?;
```

**What’s happening:** We allocate a new account, enable the Non-Transferable extension, and set up the mint (decimals, authorities). The extension must be initialized before the mint data.

---

### Phase 3: Create ATA and mint tokens

```rust
// Create Associated Token Account for the fee payer
let create_ata_instruction = create_associated_token_account(
    &fee_payer.pubkey(),
    &fee_payer.pubkey(),  // owner
    &mint.pubkey(),
    &TOKEN_2022_PROGRAM_ID,
);

// Build and send the transaction (create mint + extensions + ATA)
// ... then mint 1 token to the ATA
let mint_to_instruction = mint_to(
    &TOKEN_2022_PROGRAM_ID,
    &mint.pubkey(),
    &associated_token_address,
    &fee_payer.pubkey(),
    &[],
    1,  // amount
)?;
```

**What’s happening:** We create the associated token account for the fee payer and mint 1 token into it. At this point, the token exists but cannot be transferred.

---

### Phase 4: Demonstrate that transfers are blocked

```rust
// Attempt to transfer (will fail)
let transfer_instruction = transfer_checked(
    &TOKEN_2022_PROGRAM_ID,
    &associated_token_address,  // source
    &mint.pubkey(),
    &associated_token_address,  // destination (same as source for demo)
    &fee_payer.pubkey(),
    &[],
    1,  // amount
    0,  // decimals
)?;

match client.send_and_confirm_transaction(&transfer_transaction).await {
    Ok(sig) => println!("Transfer succeeded unexpectedly: {}", sig),
    Err(e) => println!("Transfer failed as expected:\n{:#?}", e),
}
```

**What’s happening:** We intentionally try to transfer the token. The Token-2022 program rejects it because of the Non-Transferable extension. The program prints the error to confirm the behavior.

---

### Transaction flow (summary)

```
Fee Payer (new keypair)
    ↓
Airdrop 5 SOL
    ↓
Create Mint Account (with space for NonTransferable)
    ↓
Initialize NonTransferable extension
    ↓
Initialize Mint (decimals, authorities)
    ↓
Create Associated Token Account
    ↓
Mint 1 token → ATA
    ↓
Attempt transfer → FAILS (expected)
```

---

## Project Structure

```
solana-non-tranferable/
├── Cargo.toml          # Dependencies (Solana SDK, SPL Token 2022, etc.)
├── src/
│   └── main.rs         # Main logic: mint creation, extensions, minting, transfer attempt
├── .gitignore          # Ignores target/ and .env
└── Readme.md           # This file
```

### Dependencies

| Crate | Purpose |
|-------|---------|
| `solana-client` | RPC client for Solana |
| `solana-sdk` | Core types (keypair, transaction) |
| `solana-system-interface` | `create_account` instruction |
| `spl-token-2022-interface` | Token-2022 instructions and extensions |
| `spl-associated-token-account-interface` | ATA creation |
| `anyhow` | Error handling |
| `tokio` | Async runtime |

---

## Expected Output

Example output:

```
Fee payer address : 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
Mint Address: 9abc123...

Extensions enabled: [NonTransferable]

NonTransferable {
    // No additional data - the extension type itself blocks transfers
}

Attempting to transfer non-transferable token
Transfer failed as expected:
Error { ... }
```

---

## References

- [Token-2022 Program](https://spl.solana.com/token-2022)
- [Token Extensions Overview](https://solana.com/developers/guides/token-extensions/getting-started)
- [Non-Transferable Tokens Guide](https://solana.com/developers/guides/token-extensions/non-transferable-tokens)
