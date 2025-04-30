# Hospital Data Management Contract

## Overview

The Hospital Data Management Contract is a Soroban smart contract built on the Stellar blockchain that enables secure, transparent, and decentralized management of hospital information. This contract provides a comprehensive system for registering, updating, and querying hospital data with robust access control mechanisms.

## Features

- **Hospital Registration**: Register hospitals with essential information including name, address, license number, specialties, and capacity.
- **Access Control**: Role-based access control system with Admin, Operator, and Viewer roles.
- **Data Management**: Update hospital information, add specialties, and modify capacity.
- **Search Functionality**: Search hospitals by specialty.
- **Statistics**: Generate statistics about registered hospitals.
- **Logical Deletion**: Mark hospitals as inactive without removing data.
- **License Verification**: Validate hospital license numbers.

## Contract Structure

The contract is organized into several modules:

- **lib.rs**: Main entry point that exports the contract modules.
- **hospital.rs**: Core hospital management functionality.
- **access_control.rs**: Role-based access control system.
- **utils.rs**: Helper functions for data validation and statistics.
- **test.rs**: Comprehensive tests for contract functionality.

## Data Models

### Hospital

```rust
pub struct Hospital {
    pub id: u32,
    pub name: String,
    pub address: String,
    pub license_number: String,
    pub specialties: Vec<String>,
    pub capacity: u32,
    pub admin: Address,
    pub active: bool,
}
```

### HospitalStats

```rust
pub struct HospitalStats {
    pub total_hospitals: u32,
    pub active_hospitals: u32,
    pub total_capacity: u32,
    pub specialty_counts: Map<String, u32>,
}
```

## API Reference

### Hospital Management

- `register_hospital(env, name, address, license_number, specialties, capacity, admin) -> u32`: Register a new hospital and return its ID.
- `update_hospital(env, id, name, address, license_number, capacity) -> bool`: Update an existing hospital's information.
- `remove_hospital(env, id) -> bool`: Mark a hospital as inactive (logical deletion).
- `get_hospital(env, id) -> Hospital`: Get a specific hospital by ID.
- `list_hospitals(env) -> Vec<Hospital>`: List all active hospitals.

### Specialty Management

- `add_specialty(env, id, specialty) -> bool`: Add a new specialty to a hospital.
- `search_by_specialty(env, specialty) -> Vec<Hospital>`: Find hospitals by specialty.

### Administrative Functions

- `update_capacity(env, id, capacity) -> bool`: Update a hospital's capacity.
- `transfer_admin(env, id, new_admin) -> bool`: Transfer hospital admin rights.
- `verify_license(env, license_number) -> bool`: Verify a hospital's license.
- `get_hospital_stats(env) -> HospitalStats`: Get statistics about hospitals.

### Access Control

- `initialize(env, admin)`: Initialize access control with contract admin.
- `grant_role(env, address, role)`: Grant a role to an address.
- `revoke_role(env, address, role)`: Revoke a role from an address.
- `has_role(env, address, role) -> bool`: Check if an address has a specific role.
- `require_role(env, address, role)`: Require an address to have a specific role, panic if not.
- `get_role_members(env, role) -> Vec<Address>`: Get all addresses with a specific role.

## Building and Testing

### Prerequisites

- Rust and Cargo
- Soroban CLI

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Build for Deployment

```bash
stellar contract build
```

## Security Considerations

- Role-based access control ensures only authorized users can modify data
- Hospital admins can only modify their own hospital data
- License verification adds an additional layer of validation
- All data is stored on-chain for transparency and auditability
