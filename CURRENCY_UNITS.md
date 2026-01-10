# LuxTensor Currency Units Guide

## Overview

LuxTensor uses a precise denomination system for its native token, designed to support the ModernTensor ecosystem while competing with BitTensor's SubTensor implementation.

## Currency Units

### MDT (ModernTensor)
- **Full Name**: ModernTensor
- **Symbol**: MDT
- **Description**: The main unit of currency in LuxTensor
- **Usage**: User-facing denomination for transactions, staking, and rewards

### LTS (LuxTensor Smallest)
- **Full Name**: LuxTensor Smallest
- **Symbol**: LTS
- **Description**: The smallest indivisible unit of currency (base unit)
- **Usage**: Internal representation of all balances and amounts

## Conversion

The relationship between MDT and LTS is:

```
1 MDT = 10^18 LTS = 1,000,000,000,000,000,000 LTS
```

This 18-decimal system is compatible with Ethereum's wei/ether system and allows for:
- Precise fractional amounts
- High transaction granularity
- Compatibility with existing blockchain tooling
- Efficient internal representation

## Examples

### Common Amounts

| MDT | LTS |
|-----|-----|
| 1 MDT | 1,000,000,000,000,000,000 LTS |
| 0.1 MDT | 100,000,000,000,000,000 LTS |
| 0.01 MDT | 10,000,000,000,000,000 LTS |
| 0.001 MDT | 1,000,000,000,000,000 LTS |
| 10 MDT | 10,000,000,000,000,000,000 LTS |
| 32 MDT | 32,000,000,000,000,000,000 LTS |

### Network Parameters (in LTS)

- **Minimum Validator Stake**: 10 MDT = 10,000,000,000,000,000,000 LTS
- **Recommended Validator Stake**: 32 MDT = 32,000,000,000,000,000,000 LTS
- **Block Reward**: 2 MDT = 2,000,000,000,000,000,000 LTS
- **Gas Price (typical)**: 1 gwei = 1,000,000,000 LTS

## Programming with Currency Units

### Using the Currency Module

The `luxtensor-core` crate provides utilities for working with currency units:

```rust
use luxtensor_core::currency::{mdt_to_lts, lts_to_mdt, format_lts_as_mdt};

// Convert MDT to LTS
let amount_mdt = 10;
let amount_lts = mdt_to_lts(amount_mdt);
// amount_lts = 10,000,000,000,000,000,000

// Convert LTS to MDT (truncates fractional part)
let amount_lts = 1_500_000_000_000_000_000;
let amount_mdt = lts_to_mdt(amount_lts);
// amount_mdt = 1

// Format for display
let formatted = format_lts_as_mdt(amount_lts);
// formatted = "1.500000000000000000 MDT"
```

### Parsing User Input

```rust
use luxtensor_core::currency::parse_mdt_to_lts;

// Parse MDT string to LTS
let amount_lts = parse_mdt_to_lts("10.5").unwrap();
// amount_lts = 10,500,000,000,000,000,000
```

### Balance Storage

All balances in the blockchain are stored as `u128` values representing LTS:

```rust
pub struct Account {
    pub nonce: u64,
    pub balance: u128,  // Balance in LTS
    pub storage_root: Hash,
    pub code_hash: Hash,
}
```

## Best Practices

### For Users

1. **Always specify the unit**: When displaying amounts, clearly indicate whether it's MDT or LTS
2. **Use MDT for readability**: Display user-facing amounts in MDT with appropriate decimal places
3. **Verify calculations**: When converting between units, double-check the math to avoid errors

### For Developers

1. **Store in LTS**: Always store balances and amounts as LTS internally
2. **Convert for display**: Convert to MDT only when displaying to users
3. **Use the currency module**: Leverage the provided utility functions for conversions
4. **Check for overflow**: When multiplying or adding amounts, use saturating operations
5. **Document units**: Always document whether a value is in LTS or MDT in comments

## Common Pitfalls

### ❌ Incorrect: Mixing Units

```rust
// DON'T DO THIS - mixing units without conversion
let balance_lts = 1_000_000_000_000_000_000;
let reward_mdt = 2;
let total = balance_lts + reward_mdt; // WRONG!
```

### ✅ Correct: Convert Before Operations

```rust
// DO THIS - convert to same unit first
let balance_lts = 1_000_000_000_000_000_000;
let reward_mdt = 2;
let reward_lts = mdt_to_lts(reward_mdt);
let total = balance_lts + reward_lts; // Correct!
```

### ❌ Incorrect: Division Without Precision Loss Warning

```rust
// This loses precision - fractional MDT is truncated
let amount_lts = 1_500_000_000_000_000_000;
let amount_mdt = amount_lts / LTS_PER_MDT; // 1 (loses 0.5 MDT)
```

### ✅ Correct: Keep in LTS or Handle Precision

```rust
// Keep calculations in LTS for precision
let amount_lts = 1_500_000_000_000_000_000;
let half = amount_lts / 2; // Still precise in LTS

// Or format with decimals for display
let formatted = format_lts_as_mdt(amount_lts); // "1.500000000000000000 MDT"
```

## Historical Context

LuxTensor was designed as the blockchain layer for ModernTensor, competing with BitTensor's SubTensor. The currency unit system was chosen to:

1. **Provide precision**: 18 decimals allow for micro-transactions
2. **Ensure compatibility**: Similar to Ethereum's system for easier integration
3. **Support AI workloads**: Fine-grained rewards for AI validation tasks
4. **Enable scalability**: Efficient internal representation

## Related Documentation

- [README.md](README.md) - Main project documentation
- [Genesis Configuration](genesis.testnet.json) - Testnet token configuration
- [Currency Module API](crates/luxtensor-core/src/currency.rs) - Code implementation

## Questions?

If you have questions about currency units or need help with conversions:

1. Check the [currency module documentation](crates/luxtensor-core/src/currency.rs)
2. Review code examples in the integration tests
3. Open an issue on [GitHub](https://github.com/sonson0910/luxtensor/issues)

---

**Remember**: When in doubt, work in LTS internally and convert to MDT only for display!
