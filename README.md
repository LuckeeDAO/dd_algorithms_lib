# Decentralized Decision Library

A library for decentralized decision making, fair division algorithms, and random number generation. This library focuses on consensus-friendly, objective algorithms.

## Features

### ⚖️ Fair Division Algorithms
- Equal weights and weighted fair division
- Super fair division algorithms
- Optimal resource allocation

### 🎲 Decentralized Random Number Generation
- Single and multiple random number generation using XOR operations
- Equal probability distribution when input size is power of 2
- Collision resistance and uniqueness guarantees
- Whitelist support for exclusion-based selection

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dd_algorithms_lib = "0.1.0"
```

### Basic Usage

```rust
use dd_algorithms_lib::{
    // Fair Division
    calculate_fair_division_equal_weights,
    calculate_fair_division_weighted,
    
    // Random Generation
    get_one_dd_rand_num,
    get_one_dd_3d_rand_num,
    get_k_dd_rand_num,
    get_k_dd_rand_num_with_whitelist,
};

// Fair division example
let bids = [100i128, 200, 300];
let mut allocation = [0i128; 3];
calculate_fair_division_equal_weights(&bids, &mut allocation).unwrap();
// allocation: [66, 133, -199] (sum = 0)

// Single random number generation (XOR-based)
let values = [100u128, 200, 300, 400]; // 4 values (2^2)
let n = values.len();
let mut result = 0u128;
get_one_dd_rand_num(&values, n, &mut result).unwrap();
// result = 100 ^ 200 ^ 300 ^ 400

// Multiple random selection (XOR-based)
let group1 = [100u128, 200, 300];
let group2 = [150u128, 250, 350];
let group3 = [120u128, 220, 320];
let group4 = [130u128, 230, 330]; // 4 groups (2^2)
let groups = [group1.as_slice(), group2.as_slice(), group3.as_slice(), group4.as_slice()];
let mut selected = [0usize; 3];
get_k_dd_rand_num(&groups, 4, 3, &mut selected).unwrap();
// selected: [1, 2, 0] (unique participant indices)
```

## Modules

### `algorithms`
Mathematical algorithms for fair division and random number generation:
- `calculate_fair_division_equal_weights()` - Fair division with equal weights
- `calculate_fair_division_weighted()` - Fair division with custom weights
- `get_one_dd_rand_num()` - Generate single decentralized random number (XOR-based)
- `get_one_dd_3d_rand_num()` - Generate single random number for lottery (XOR-based)
- `get_k_dd_rand_num()` - Generate multiple unique random numbers (XOR-based)
- `get_k_dd_rand_num_with_whitelist()` - Generate multiple random numbers with exclusions (XOR-based)

### `types`
Common data types and enums:
- `VotingPower` - Voting power type alias
- `ParticipantId` - Participant identifier type
- `Timestamp` - Timestamp type alias
- `FairDivisionResult` - Result structure for fair division (if used)
- `RandomSelectionResult` - Result structure for random selection (if used)

## Algorithm Details

### Fair Division
Implements super fair division algorithms that ensure:
- Zero-sum allocations (sum of all allocations equals zero)
- Fair distribution based on input values or weights
- Optimal resource allocation

### XOR-based Random Number Generation
The library uses XOR (exclusive OR) operations for random number generation, providing several advantages:

#### Why XOR?
- **Equal Probability**: When input size is power of 2, XOR ensures each possible result has equal probability
- **Efficiency**: XOR operations are faster than modular arithmetic
- **Commutativity**: Order of inputs doesn't affect the result (A ⊕ B = B ⊕ A)
- **Associativity**: Grouping doesn't affect the result ((A ⊕ B) ⊕ C = A ⊕ (B ⊕ C))
- **Deterministic**: Same inputs always produce same output (important for consensus)

#### Mathematical Properties
- XOR is its own inverse: A ⊕ A = 0
- XOR with zero is identity: A ⊕ 0 = A
- XOR distributes over itself: A ⊕ (B ⊕ C) = (A ⊕ B) ⊕ C

### Random Number Generation
Decentralized random number generation using XOR operations with:
- **XOR-based Algorithm**: Uses bitwise XOR for equal probability distribution
- **Power of 2 Constraint**: Input size must be power of 2 for optimal randomness
- **Collision Resistance**: Ensures all generated numbers are unique
- **Unpredictability**: Uses participant-provided random values
- **Offset Mechanism**: Prevents patterns through systematic offsetting
- **Whitelist Support**: Exclude specific participants from selection
- **Validation**: Comprehensive parameter validation and bounds checking

## Examples

See the `examples/` directory for comprehensive usage examples:
- `governance_example.rs` - Complete demonstration of all features
- `function_names_test.rs` - Function name verification and testing

## Constraints

- **Participants (n)**: ≤ 100,000 and must be power of 2 (2^n)
- **Selections (k)**: ≤ 1,000
- **k ≤ n**: Cannot select more participants than available
- **XOR Algorithm**: Requires power of 2 input size for equal probability distribution

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Publishing to crates.io

Follow this checklist to publish a new version of this crate to crates.io.

### 1) Prerequisites
- Create and verify a crates.io account
- (Recommended) Enable 2FA on crates.io
- Create an API token on crates.io → Account → API Tokens
- Login locally:

```bash
cargo login <YOUR_API_TOKEN>
```

### 2) Verify Cargo.toml metadata
Ensure the following fields are correct: `name`, `version`, `description`, `license`, `repository`, `documentation`, `readme`, `keywords`, `categories`, `rust-version`. This crate defaults to `no_std` and offers optional `std`, `serde`, and `log_tests` features.

### 3) Build, test, docs

```bash
cargo clean
cargo test
# Optional: show test logs
cargo test --features log_tests -- --nocapture
# Local docs
cargo doc --no-deps
```

### 4) Package and dry run

```bash
cargo package
cargo publish --dry-run
```

### 5) Publish

```bash
cargo publish
```

### 6) Versioning and tags

```bash
# Bump version in Cargo.toml first
git tag -a v0.1.0 -m "Release v0.1.0"
git push --tags
```

### 7) Manage owners

```bash
cargo owner --add <github-user-or-team>
cargo owner --list
```

### 8) Troubleshooting
- If packaged content is wrong, inspect with `cargo package` and adjust `include`/`exclude` in `Cargo.toml` or `.gitignore`.
- For `no_std` docs on docs.rs, ensure default features don’t pull in `std`. This crate uses `#![no_std]` by default.
- Printing in tests is gated behind the optional `log_tests` feature to keep default `no_std` behavior.

### 9) Releasing fixes
Crates cannot be overwritten. Bump the version and republish. You can yank a bad version:

```bash
cargo yank --vers <version>
# Undo if necessary
cargo yank --vers <version> --undo
```