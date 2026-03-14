# Metalend Security Audit Report

## 1. Supply — Check Zero Amount

**Severity**: Low  
**Location**: `instructions/supply.rs:10-11`

**Description**:  
The supply instruction doesn’t reject `amount == 0`. Supplying 0 tokens might waste gas.

**Impact**:  
Low — mostly unnecessary transactions and possible edge cases.

**Recommendation**:  
Add at the start of the instruction:

```rust
require!(amount > 0, LendingError::InvalidAmount);
```

---

## 2. Create Market — Oracles Not Validated

**Severity**: Medium  
**Location**: `contexts.rs:34-36`, `instructions/market.rs`

**Description**:  
When creating a market, `supply_oracle` and `collateral_oracle` are passed as unchecked `AccountInfo`. The protocol doesn’t verify they’re real oracle accounts (correct program, correct format). A market creator can point to any account.

**Impact**:  
A malicious market creator can use fake oracles they control. They could:

- Create markets with arbitrary prices
- Allow over borrowing
- Trigger wrongful liquidations

**Recommendation**:

- Validate that oracle accounts are PDA-derived from the program (or from a known oracle program).
- Deserialize and sanity-check oracle data (e.g. mint, price range, etc.) before using.

## 3. Close User Deposit — No Owner Check

**Severity**: Medium  
**Location**: `contexts.rs:305-313`

**Description**:  
`CloseUserDeposit` sends the account’s lamports to the `user` signer, but there is no check that `user_deposit.user == user.key()`. As a result, someone could pass another user’s empty deposit account, sign as themselves, and close it — receiving that user’s rent.

**Impact**:

- Rent from other users’ empty deposit accounts can be stolen
- This is not exactly a bug but we can sure about.

**Proof of Concept**:

```typescript
// Attacker closes victim's empty user deposit, receives rent
await program.methods
  .closeUserDeposit()
  .accounts({
    userDeposit: victimUserDeposit, // Victim's account
    user: attacker.publicKey, // Attacker receives the lamports!
  })
  .signers([attacker])
  .rpc();
```

**Recommendation**:  
Add `has_one = user` to the `user_deposit` account constraint so only the owner can close it:

```rust
#[account(
    mut,
    close = user,
    has_one = user,
)]
pub user_deposit: Account<'info, UserDeposit>,
```

---

## 4. Update Market Params — Anyone Can Change Risk Settings

**Severity**: High  
**Location**: `/market_admin.rs`, `contexts.rs:326-331`

**Description**:  
The `update_market_params` instruction lets you change the collateral factor and liquidation threshold of any market. The problem is it only checks that _someone_ signs the transaction — it never checks that the signer is actually the market admin.

So literally anyone can call this and change these critical risk parameters. For example, an attacker could set collateral factor to 100% (so we could borrow way more than we should) or mess with liquidation thresholds to make healthy positions look liquidatable (or the opposite).

**Impact**:  
Attackers can change market risk parameters without permission. This can lead to:

- Over-borrowing (bad debt)
- Unfair liquidations
- Protocol insolvency

**Proof of Concept**:

```typescript
// Attacker (not the market admin) changes params
await program.methods
  .updateMarketParams(
    new anchor.BN(10000), // 100% collateral factor - borrow everything!
    new anchor.BN(9900), // 99% liquidation - liquidate almost everyone
  )
  .accounts({
    market,
    authority: attacker.publicKey, // NOT the market admin!
  })
  .signers([attacker])
  .rpc();
// This would succeed even though attacker isn't the admin
```

**Recommendation**:  
Add a constraint so only the market admin can call this. In the `UpdateMarketParams` struct, add:

```rust
#[account(
    mut,
    has_one = authority,
    constraint = market.market_admin == authority.key()
)]
pub market: Account<'info, Market>,
```

Or use `has_one = authority` and ensure the market's `market_admin` matches the signer.

---

## 2. Liquidation Uses Units / Wrong Oracle

**Severity**: High  
**Location**: `instructions/liquidate.rs:21-42`

**Description**:  
The liquidation logic uses a single oracle for both collateral value and borrow value. But in a dual-asset market, collateral is one token (e.g. ETH) and borrowed debt is another (e.g. USDC). we need two oracles: one for collateral price and one for borrow asset price.

Right now:

- `collateral_value` = collateral_deposited × oracle_price (correct if oracle is for collateral)
- `borrow_value` = borrowed_amount × oracle_price (wrong if same oracle — borrowed_amount is in supply token, not collateral!)

So if we pass the collateral oracle (e.g. ETH price), we're multiplying USDC amounts by ETH price, which gives garbage numbers.

Additionally, `collateral_to_seize` is computed as:

```rust
collateral_to_seize = liquidation_amount * liquidation_bonus / 1000
```

`liquidation_amount` is in supply token (USDC) and `collateral_to_seize` should be in collateral token (ETH). The code mixes units without converting. We need to convert using both prices: how much collateral (in collateral units) equals the debt repaid plus the bonus.

**Impact**:

- Wrong liquidation decisions (liquidate when healthy, or skip unhealthy positions)
- Wrong amount of collateral seized (could seize too much or too little)
- Protocol insolvency or unfair liquidations

**Proof of Concept**:  
Existing tests use `ethOracle` for liquidation. With that, `borrow_value` = 200 USDC × 3000 (ETH price) = 600,000 — which is not a valid comparison to collateral value.

**Recommendation**:

- Pass two oracles: one for collateral, one for supply/borrow.
- Compute collateral_value = collateral_deposited × collateral_price.
- Compute borrow_value = borrowed_amount × supply_price.
- For collateral_to_seize, convert from supply to collateral:  
  `collateral_to_seize = (liquidation_amount * supply_price / collateral_price) * (1 + bonus)`

---
