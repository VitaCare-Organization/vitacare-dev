// SPDX-License-Identifier: MIT
use soroban_sdk::{contract, contractimpl, vec, Address, Env, String, Symbol};

mod types;
mod storage;

use types::{PatientData, PatientRegistry};
use storage::Storage;

#[contract]
pub struct PatientIdentityContract;

#[contractimpl]
impl PatientIdentityContract {
    /// Initialize the contract with an admin
    pub fn initialize(env: Env, admin: Address) {
        // Ensure the contract isn't already initialized
        if Storage::has_registry(&env) {
            panic!("Contract is already initialized");
        }

        // Verify admin authorization
        admin.require_auth();

        // Initialize registry
        let registry = PatientRegistry {
            admin,
            patients: vec![&env],
        };
        Storage::set_registry(&env, &registry);
    }

    /// Register a new patient
    pub fn register_patient(
        env: Env,
        wallet: Address,
        name: String,
        dob: u64,
        metadata: String,
    ) -> PatientData {
        // Require authorization from the wallet being registered
        wallet.require_auth();

        // Check if patient already exists
        if Storage::has_patient(&env, &wallet) {
            panic!("Patient already registered");
        }

        // Create and store patient data
        let now = env.ledger().timestamp();
        let patient_data = PatientData {
            name,
            dob,
            metadata,
            registered_at: now,
            last_updated: now,
        };
        Storage::set_patient(&env, &wallet, &patient_data);

        // Update the registry
        let mut registry: PatientRegistry = Storage::get_registry(&env).unwrap();
        registry.patients.push_back(wallet.clone());
        Storage::set_registry(&env, &registry);

        // Emit registration event
        env.events().publish(
            (Symbol::new(&env, "patient_registered"), wallet.clone()),
            patient_data.clone(),
        );

        patient_data
    }

    /// Update an existing patient's information
    pub fn update_patient(env: Env, wallet: Address, metadata: String) -> PatientData {
        // Require authorization from the patient's wallet
        wallet.require_auth();

        // Get existing patient data
        let mut patient_data: PatientData = Storage::get_patient(&env, &wallet)
            .unwrap_or_else(|| panic!("Patient not found"));

        // Update metadata and timestamp
        patient_data.metadata = metadata;
        patient_data.last_updated = env.ledger().timestamp();

        // Store updated data
        Storage::set_patient(&env, &wallet, &patient_data);

        // Emit update event
        env.events().publish(
            (Symbol::new(&env, "patient_updated"), wallet.clone()),
            patient_data.clone(),
        );

        patient_data
    }

    /// Get patient data
    pub fn get_patient(env: Env, wallet: Address) -> PatientData {
        Storage::get_patient(&env, &wallet)
            .unwrap_or_else(|| panic!("Patient not found"))
    }

    /// Get all registered patients
    pub fn get_all_patients(env: Env) -> Vec<Address> {
        let registry: PatientRegistry = Storage::get_registry(&env).unwrap();
        registry.patients
    }

    /// Add insurance reference to patient metadata
    pub fn add_insurance_reference(
        env: Env,
        wallet: Address,
        insurance_id: String,
        policy_number: String,
    ) -> PatientData {
        // Require authorization from the patient's wallet
        wallet.require_auth();

        // Get existing patient data
        let mut patient_data: PatientData = Storage::get_patient(&env, &wallet)
            .unwrap_or_else(|| panic!("Patient not found"));

        // This is a simplified approach. In production, you would want to:
        // 1. Parse the existing metadata JSON
        // 2. Add/update the insurance information
        // 3. Re-serialize to JSON
        // For simplicity, we're just appending the insurance information
        let insurance_info = format!(
            "{{\"insurance_id\":\"{}\",\"policy_number\":\"{}\"}}",
            insurance_id, policy_number
        );
        patient_data.metadata = String::from_slice(&env, &format!(
            "{}|INSURANCE:{}",
            patient_data.metadata.to_string(),
            insurance_info
        ));
        patient_data.last_updated = env.ledger().timestamp();

        // Store updated data
        Storage::set_patient(&env, &wallet, &patient_data);

        // Emit insurance update event
        env.events().publish(
            (Symbol::new(&env, "insurance_added"), wallet.clone()),
            insurance_id,
        );

        patient_data
    }

    /// Add medical history reference to patient metadata
    pub fn add_medical_history_reference(
        env: Env,
        wallet: Address,
        reference_id: String,
        reference_type: String,
    ) -> PatientData {
        // Require authorization from the patient's wallet
        wallet.require_auth();

        // Get existing patient data
        let mut patient_data: PatientData = Storage::get_patient(&env, &wallet)
            .unwrap_or_else(|| panic!("Patient not found"));

        // Similar to insurance, this is a simplified approach
        let reference_info = format!(
            "{{\"reference_id\":\"{}\",\"type\":\"{}\"}}",
            reference_id, reference_type
        );
        patient_data.metadata = String::from_slice(&env, &format!(
            "{}|MEDICAL_HISTORY:{}",
            patient_data.metadata.to_string(),
            reference_info
        ));
        patient_data.last_updated = env.ledger().timestamp();

        // Store updated data
        Storage::set_patient(&env, &wallet, &patient_data);

        // Emit medical history update event
        env.events().publish(
            (Symbol::new(&env, "medical_history_added"), wallet.clone()),
            reference_id,
        );

        patient_data
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_patient_registration_and_retrieval() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);

        // Initialize contract
        let admin = Address::generate(&env);
        client.initialize(&admin);

        // Register a patient
        let patient_wallet = Address::generate(&env);
        let name = String::from_slice(&env, "John Doe");
        let dob = 946684800; // 2000-01-01
        let metadata = String::from_slice(&env, "{\"gender\":\"male\",\"blood_type\":\"A+\"}");

        let patient_data = client.register_patient(&patient_wallet, &name, &dob, &metadata);
        
        // Verify registration
        assert_eq!(patient_data.name, name);
        assert_eq!(patient_data.dob, dob);
        assert_eq!(patient_data.metadata, metadata);
        
        // Retrieve patient data
        let retrieved_data = client.get_patient(&patient_wallet);
        assert_eq!(retrieved_data, patient_data);
    }

    #[test]
    fn test_patient_update() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);

        // Initialize contract
        let admin = Address::generate(&env);
        client.initialize(&admin);

        // Register a patient
        let patient_wallet = Address::generate(&env);
        let name = String::from_slice(&env, "John Doe");
        let dob = 946684800; // 2000-01-01
        let metadata = String::from_slice(&env, "{\"gender\":\"male\",\"blood_type\":\"A+\"}");

        client.register_patient(&patient_wallet, &name, &dob, &metadata);
        
        // Update patient
        let updated_metadata = String::from_slice(&env, "{\"gender\":\"male\",\"blood_type\":\"A+\",\"allergies\":[\"penicillin\"]}");
        let updated_data = client.update_patient(&patient_wallet, &updated_metadata);
        
        // Verify update
        assert_eq!(updated_data.metadata, updated_metadata);
        
        // Retrieve updated data
        let retrieved_data = client.get_patient(&patient_wallet);
        assert_eq!(retrieved_data.metadata, updated_metadata);
    }

    #[test]
    fn test_insurance_reference() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);

        // Initialize contract
        let admin = Address::generate(&env);
        client.initialize(&admin);

        // Register a patient
        let patient_wallet = Address::generate(&env);
        let name = String::from_slice(&env, "John Doe");
        let dob = 946684800; // 2000-01-01
        let metadata = String::from_slice(&env, "{\"gender\":\"male\"}");

        client.register_patient(&patient_wallet, &name, &dob, &metadata);
        
        // Add insurance reference
        let insurance_id = String::from_slice(&env, "INS123");
        let policy_number = String::from_slice(&env, "POL456");
        
        let updated_data = client.add_insurance_reference(&patient_wallet, &insurance_id, &policy_number);
        
        // Verify insurance was added
        assert!(updated_data.metadata.to_string().contains("INSURANCE"));
        assert!(updated_data.metadata.to_string().contains("INS123"));
    }
}