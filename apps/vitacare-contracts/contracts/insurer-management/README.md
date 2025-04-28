# VitaCare Insurer Management Contract

A Soroban smart contract for the VitaCare platform that manages insurance company information, coverage policies, and claims reviewers.

## Overview

This contract enables secure and transparent storage of insurance provider information on the blockchain, allowing for the verification of insurance companies, their coverage policies, and authorized claims reviewers within the VitaCare medical claims processing platform.

## Features

- **Insurer Registration & Management**
  - Register insurance companies with identifying information
  - Update company metadata
  - Activate/deactivate insurers

- **Coverage Policy Management**
  - Add and update coverage policies
  - Track policy details including premiums, coverage amounts, and durations

- **Claims Reviewer Management**
  - Register authorized medical professionals for claims review
  - Update reviewer information and status

- **Data Security & Authorization**
  - Only the registered insurance company wallet can modify its own data
  - Secure storage with unique key generation per insurer

## Contract Functions

### Core Functions

- `register_insurer(wallet: Address, name: String, license_id: String, metadata: String)`
- `update_insurer(wallet: Address, metadata: String)`
- `get_insurer(wallet: Address) -> InsurerData`
- `deactivate_insurer(wallet: Address)`
- `reactivate_insurer(wallet: Address)`
- `get_all_insurers() -> Vec<Address>`

### Policy Management

- `add_coverage_policy(wallet: Address, id: String, name: String, description: String, premium_amount: u64, coverage_amount: u64, duration_days: u32)`
- `update_coverage_policy(wallet: Address, id: String, name: String, description: String, premium_amount: u64, coverage_amount: u64, duration_days: u32)`

### Reviewer Management

- `add_claims_reviewer(insurer_wallet: Address, reviewer_wallet: Address, name: String, role: String)`
- `update_claims_reviewer(insurer_wallet: Address, reviewer_wallet: Address, name: String, role: String, active: bool)`

## Data Structures

### InsurerData

```rust
pub struct InsurerData {
    pub wallet: Address,
    pub name: String,
    pub license_id: String,
    pub metadata: String,
    pub coverage_policies: Vec<CoveragePolicy>,
    pub claims_reviewers: Vec<ClaimsReviewer>,
    pub active: bool,
    pub registered_at: u64,
    pub updated_at: u64,
}
```

### CoveragePolicy

```rust
pub struct CoveragePolicy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub premium_amount: u64,
    pub coverage_amount: u64,
    pub duration_days: u32,
}
```

### ClaimsReviewer

```rust
pub struct ClaimsReviewer {
    pub wallet: Address,
    pub name: String,
    pub role: String,
    pub active: bool,
}
```

## Storage Design

The contract uses a `DataKey` enum to manage storage keys, providing a centralized approach to storage management with the following key types:

- `InsurerMap`: Maps wallet addresses to unique IDs
- `AllInsurers`: List of all registered insurer addresses
- `Insurer(Address)`: Individual insurer data

## Building the Contract

```bash
# Navigate to the contract directory
cd apps/vitacare-contracts/contracts/insurer-management

# Build the contract
cargo build --target wasm32-unknown-unknown --release
```

## Testing

The contract includes a comprehensive test suite covering all functionalities and edge cases:

```bash
# Run all tests
cargo test
```

## Usage in the VitaCare Platform

This contract serves as a critical component of the VitaCare medical claims processing platform by:

1. Providing verifiable registry of legitimate insurance providers
2. Ensuring transparency in coverage policies
3. Establishing on-chain records of authorized claims reviewers
4. Supporting the claims verification workflow

## License

This project is licensed under the terms of the [MIT License](LICENSE).

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md) for contribution guidelines.
