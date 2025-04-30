# Insurance Claims Smart Contract

A Soroban smart contract that automates and verifies the medical claim process with full transparency for patients and insurers.

## Features

- Patients can submit insurance claims with service details and cost
- Verified insurers can approve or reject claims
- Full transparency with claim status tracking
- Admin controls for managing verified insurers
- Secure authentication using Soroban's authorization system

## Contract Functions

### Admin Functions

- `initialize(admin: Address, insurers: Vec<Address>)`: Initialize the contract with an admin and list of verified insurers
- `add_insurer(insurer: Address)`: Add a new verified insurer (admin only)

### Patient Functions

- `submit_claim(patient: Address, service_id: u64, cost: i128) -> u64`: Submit a new insurance claim, returns the claim ID
- `get_claim_status(claim_id: u64) -> ClaimStatus`: Check the status of a specific claim
- `get_claim_details(claim_id: u64) -> Claim`: Get detailed information about a claim
- `get_patient_claims(patient: Address) -> Vec<u64>`: Get all claim IDs for a specific patient

### Insurer Functions

- `process_claim(insurer: Address, claim_id: u64, approve: bool)`: Process a claim (approve or reject)

## Claim Statuses

- `Pending`: Initial status when a claim is submitted
- `Approved`: Claim has been approved by a verified insurer
- `Rejected`: Claim has been rejected by a verified insurer

## Usage

Deploy this contract to the Soroban network and initialize it with an admin address and a list of verified insurer addresses. Once deployed, patients can submit claims, and verified insurers can process them.

## Security

The contract implements several security measures:
- Authorization checks to ensure only the patient can submit their own claims
- Verification that only approved insurers can process claims
- Admin-only functions for managing the list of verified insurers

## Testing

Comprehensive tests are included to verify all contract functionality. 