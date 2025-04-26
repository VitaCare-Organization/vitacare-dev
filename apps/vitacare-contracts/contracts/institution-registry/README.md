# Institution Registry Contract

A Soroban smart contract that allows hospitals, clinics, and private practices to register on the Stellar blockchain, with support for institution verification from trusted authorities.

## Features

- **Institution Registration**: Medical institutions can register their details securely on the blockchain
- **Data Storage**: Stores institution name, license ID, and additional metadata
- **Updates**: Institutions can update their metadata as needed
- **Verification**: Trusted authorities can verify institutions
- **Admin Control**: Only authorized admins can verify institutions

## Functions

- `register_institution(wallet: Address, name: String, license_id: String, metadata: String) -> InstitutionData`
- `get_institution(wallet: Address) -> InstitutionData`
- `update_institution(wallet: Address, metadata: String) -> InstitutionData`
- `verify_institution(admin: Address, wallet: Address) -> InstitutionData`
- `set_admin(admin: Address)`

## Data Structure

```rust
pub struct InstitutionData {
    name: String,
    license_id: String,
    metadata: String,
    verified: bool,
}
```

## Security Features

- Authentication checks for all operations
- Only the institution itself can update its data
- Only authorized admins can verify institutions

## Building and Testing

```shell
# Build the contract
cargo build --release

# Run the tests
cargo test
```

## Deployment

Deploy the contract using the Stellar CLI:

```shell
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/vitacare_institution_registry.wasm \
  --source <your-stellar-address> \
  --rpc-url <rpc-url> \
  --network-passphrase <network-passphrase>
``` 