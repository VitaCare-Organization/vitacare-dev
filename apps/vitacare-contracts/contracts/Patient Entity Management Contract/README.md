# Medical Record Management Smart Contract

## Overview

This smart contract allows patients to securely store and manage their medical records on the Soroban blockchain platform. It empowers patients with complete control over their medical data, allowing them to grant and revoke access to healthcare providers as needed.

## Key Features

- **Patient-Controlled Access**: Patients have full control over who can access their medical records
- **Secure Storage**: Only stores metadata and hashes of medical records, with actual data stored off-chain (e.g., IPFS)
- **Privacy-Preserving**: Each patient's records are completely separated from others
- **Selective Sharing**: Patients can individually authorize and revoke access for each healthcare provider
- **Comprehensive History**: Maintains a complete history of medical records even after doctor access changes

## Architecture

The smart contract is built on the Soroban platform using Rust, and consists of the following main components:

### Core Data Structures

- **RecordMetadata**: Stores essential metadata about a medical record (ID, patient, doctor, data hash, description)
- **Storage Maps**: Efficiently stores patient records, authorized doctors, and record counters

### Main Functions

1. **Medical Record Management**
   - `add_medical_record`: Add a new medical record for a patient
   - `get_medical_records`: Retrieve all medical records for a patient

2. **Access Control**
   - `grant_access`: Allow a doctor to access a patient's records
   - `revoke_access`: Remove a doctor's access to a patient's records
   - `get_authorized_doctors`: List all doctors authorized to access a patient's records

## Use Cases

### For Patients

- Securely store and manage their complete medical history
- Control which healthcare providers can access their records
- Add self-reported symptoms or conditions
- Maintain continuous medical history even when changing doctors

### For Healthcare Providers

- Access comprehensive patient medical history (with permission)
- Add new medical records for patients under their care
- Collaborate with other authorized healthcare providers on patient care

## Privacy and Security Considerations

This contract implements several key privacy and security features:

1. **Data Segregation**: Each patient's records are stored separately
2. **Minimal On-Chain Data**: Only metadata and hashes are stored on-chain
3. **Authorization Controls**: Only authorized doctors can add or view records
4. **Patient Sovereignty**: Patients can revoke access at any time

## Technical Implementation

The contract is built on the Soroban platform using Rust and includes:

- Efficient storage mechanisms using Soroban's persistent storage features
- Comprehensive testing covering all core functions and edge cases
- Clear separation of concerns between storage, access control, and record management

## Testing

The contract includes extensive tests covering:

- Basic medical record flow
- Multiple doctor access scenarios
- Separation of patient records
- Patient self-reporting
- Complex medical histories
- Doctor access revocation and reauthorization

## Getting Started

To deploy this contract:

1. Build the contract with `cargo build`
2. Deploy the contract to a Soroban network
3. Interact with the contract using the Soroban SDK or CLI

## Example Usage

```rust
// Initialize contract
let contract = MedicalRecordsContract::new();

// Patient grants access to a doctor
contract.grant_access(&patient_address, &doctor_address);

// Doctor adds a medical record
contract.add_medical_record(
    &patient_address,
    &doctor_address,
    &record_hash,
    &description
);

// Patient retrieves their records
let records = contract.get_medical_records(&patient_address);

// Patient revokes doctor access when no longer needed
contract.revoke_access(&patient_address, &doctor_address);
```

---