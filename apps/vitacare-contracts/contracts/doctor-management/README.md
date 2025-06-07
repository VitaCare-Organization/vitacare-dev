# Doctor Management Smart Contract

This Soroban (Stellar) smart contract provides a solution for managing doctor profiles on the VitaCare platform. It enables secure and efficient creation, updating, and querying of doctor profiles on the blockchain.

## Features

- Doctor profile creation with basic information
- Specialization and metadata updates
- Credential verification
- Medical institution linking
- Timestamp management for auditing
- Robust error handling

## Contract Structure

### Data Types

```rust
pub struct DoctorProfile {
    pub wallet: Address,           // Doctor's wallet address
    pub name: String,              // Doctor's name
    pub specialization: String,    // Medical specialization
    pub institution_wallet: Address, // Institution's address
    pub metadata: String,          // Additional metadata (certifications, etc.)
    pub created_at: u64,          // Creation timestamp
    pub updated_at: u64,          // Last update timestamp
}
```

### Core Functions

1. `create_doctor_profile`
   - Creates a new doctor profile
   - Requires wallet, name, specialization, and institution
   - Prevents duplicates

2. `update_doctor_profile`
   - Updates specialization and/or metadata
   - Maintains change history
   - Updates timestamp

3. `get_doctor_profile`
   - Queries doctor information
   - Returns complete profile

### Error Handling

- `DoctorNotFound`: Profile doesn't exist
- `DoctorAlreadyExists`: Duplicate attempt
- `InvalidInput`: Invalid data

## Usage

### Create Profile

```rust
contract.create_doctor_profile(
    &doctor_wallet,
    &String::from_str(&env, "Dr. John Doe"),
    &String::from_str(&env, "Cardiology"),
    &institution_wallet
);
```

### Update Profile

```rust
contract.update_doctor_profile(
    &doctor_wallet,
    &Some(String::from_str(&env, "Pediatric Cardiology")),
    &Some(String::from_str(&env, "Board Certified"))
);
```

### Query Profile

```rust
let profile = contract.get_doctor_profile(&doctor_wallet);
```

## Testing

The contract includes a comprehensive test suite covering:

- Profile creation
- Duplicate prevention
- Profile updates
- Error handling
- Multiple doctors per institution
- Metadata persistence
- Timestamp validation

To run the tests:

```bash
cargo test
```

## Security

- Input validation
- Duplicate prevention
- Timestamps for auditing
- Robust error handling
- Controlled function access

## Development

### Requirements

- Rust 1.70.0 or higher
- Soroban CLI
- Stellar Development Environment

### Building

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Deployment

```bash
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/doctor_management.wasm \
    --source <SOURCE_ACCOUNT> \
    --rpc-url <RPC_URL> \
    --network-passphrase <NETWORK_PASSPHRASE>
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

VitaCare Team
