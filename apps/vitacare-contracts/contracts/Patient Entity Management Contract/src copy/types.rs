// SPDX-License-Identifier: MIT
use soroban_sdk::{contracttype, Env, String, Vec, Address, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatientData {
    pub name: String,
    pub dob: u64,  // Unix timestamp for date of birth
    pub metadata: String,  // JSON string containing additional patient data
    pub registered_at: u64,
    pub last_updated: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct PatientRegistry {
    pub admin: Address,
    pub patients: Vec<Address>,
}