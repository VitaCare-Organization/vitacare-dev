#![no_std]
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Map, Vec};

#[derive(Clone)]
#[contracttype]
pub struct PatientData {
    name: String,
    dob: u64,
    metadata: String,
    insurance_ref: Option<String>,
    medical_history_ref: Option<String>,
    last_updated: u64,
}

#[contract]
pub struct PatientIdentityContract;

#[contractimpl]
impl PatientIdentityContract {
    // Register a new patient with basic information
    pub fn register_patient(
        env: Env,
        wallet: Address,
        name: String,
        dob: u64,
        metadata: String,
    ) -> Result<(), String> {
        // Verify caller is the wallet owner for authentication
        wallet.require_auth();
        
        // Check if patient already exists
        let patients = env.storage().persistent();
        if patients.has(&wallet) {
            return Err(String::from_str(&env, "Patient already registered"));
        }
        
        // Create patient record
        let patient = PatientData {
            name,
            dob,
            metadata,
            insurance_ref: None,
            medical_history_ref: None,
            last_updated: env.ledger().timestamp(),
        };
        
        // Store patient data
        patients.set(&wallet, &patient);
        
        // Emit patient registration event
        env.events().publish(("patient_registered", wallet.clone()), wallet);
        
        Ok(())
    }
    
    // Update patient information
    pub fn update_patient(
        env: Env,
        wallet: Address,
        metadata: String,
    ) -> Result<(), String> {
        // Verify caller is the wallet owner
        wallet.require_auth();
        
        // Get patient data
        let patients = env.storage().persistent();
        if !patients.has(&wallet) {
            return Err(String::from_str(&env, "Patient not found"));
        }
        
        let mut patient: PatientData = patients.get(&wallet).unwrap();
        
        // Update fields
        patient.metadata = metadata;
        patient.last_updated = env.ledger().timestamp();
        
        // Store updated data
        patients.set(&wallet, &patient);
        
        // Emit update event
        env.events().publish(("patient_updated", wallet.clone()), wallet);
        
        Ok(())
    }
    
    // Get patient information
    pub fn get_patient(env: Env, wallet: Address) -> Result<PatientData, String> {
        // Get patient data
        let patients = env.storage().persistent();
        if !patients.has(&wallet) {
            return Err(String::from_str(&env, "Patient not found"));
        }
        
        // Return the patient data
        Ok(patients.get(&wallet).unwrap())
    }
    
    // Add insurance reference
    pub fn add_insurance(
        env: Env,
        wallet: Address,
        insurance_ref: String,
    ) -> Result<(), String> {
        // Verify caller is the wallet owner
        wallet.require_auth();
        
        // Get patient data
        let patients = env.storage().persistent();
        if !patients.has(&wallet) {
            return Err(String::from_str(&env, "Patient not found"));
        }
        
        let mut patient: PatientData = patients.get(&wallet).unwrap();
        
        // Update insurance reference
        patient.insurance_ref = Some(insurance_ref);
        patient.last_updated = env.ledger().timestamp();
        
        // Store updated data
        patients.set(&wallet, &patient);
        
        // Emit insurance update event
        env.events().publish(("insurance_updated", wallet.clone()), wallet);
        
        Ok(())
    }
    
    // Add medical history reference
    pub fn add_medical_history(
        env: Env,
        wallet: Address,
        medical_history_ref: String,
    ) -> Result<(), String> {
        // Verify caller is the wallet owner
        wallet.require_auth();
        
        // Get patient data
        let patients = env.storage().persistent();
        if !patients.has(&wallet) {
            return Err(String::from_str(&env, "Patient not found"));
        }
        
        let mut patient: PatientData = patients.get(&wallet).unwrap();
        
        // Update medical history reference
        patient.medical_history_ref = Some(medical_history_ref);
        patient.last_updated = env.ledger().timestamp();
        
        // Store updated data
        patients.set(&wallet, &patient);
        
        // Emit medical history update event
        env.events().publish(("medical_history_updated", wallet.clone()), wallet);
        
        Ok(())
    }
    
    // Grant access to a healthcare provider (optional)
    pub fn grant_access(
        env: Env,
        patient_wallet: Address,
        provider_wallet: Address,
        expiration: u64,
    ) -> Result<(), String> {
        // Verify caller is the patient
        patient_wallet.require_auth();
        
        // Get access control map
        let access_map_key = String::from_str(&env, "access_control");
        let mut access_map: Map<Address, Map<Address, u64>> = env.storage()
            .persistent()
            .get(&access_map_key)
            .unwrap_or_else(|| Map::new(&env));
        
        // Get or create provider map for this patient
        let mut provider_map = access_map.get(&patient_wallet)
            .unwrap_or_else(|| Map::new(&env));
        
        // Set expiration for provider
        provider_map.set(&provider_wallet, &expiration);
        
        // Update maps
        access_map.set(&patient_wallet, &provider_map);
        env.storage().persistent().set(&access_map_key, &access_map);
        
        // Emit access granted event
        env.events().publish(
            ("access_granted", patient_wallet.clone(), provider_wallet.clone()),
            expiration
        );
        
        Ok(())
    }
    
    // Check if a provider has access to a patient's data
    pub fn check_access(
        env: Env,
        patient_wallet: Address,
        provider_wallet: Address,
    ) -> Result<bool, String> {
        // Get access control map
        let access_map_key = String::from_str(&env, "access_control");
        let access_map: Map<Address, Map<Address, u64>> = env.storage()
            .persistent()
            .get(&access_map_key)
            .unwrap_or_else(|| Map::new(&env));
        
        // Get provider map for this patient
        if let Some(provider_map) = access_map.get(&patient_wallet) {
            if let Some(expiration) = provider_map.get(&provider_wallet) {
                // Check if access hasn't expired
                return Ok(expiration > env.ledger().timestamp());
            }
        }
        
        Ok(false)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};
    
    #[test]
    fn test_patient_registration() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        
        let user = Address::generate(&env);
        let name = String::from_str(&env, "John Doe");
        let dob = 631152000; // Unix timestamp for Jan 1, 1990
        let metadata = String::from_str(&env, "{\"gender\":\"M\",\"blood_type\":\"O+\"}");
        
        // Register patient
        client.register_patient(&user, &name, &dob, &metadata);
        
        // Get patient data
        let patient_data = client.get_patient(&user).unwrap();
        assert_eq!(patient_data.name, name);
        assert_eq!(patient_data.dob, dob);
        assert_eq!(patient_data.metadata, metadata);
    }
    
    #[test]
    fn test_patient_update() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        
        let user = Address::generate(&env);
        let name = String::from_str(&env, "John Doe");
        let dob = 631152000;
        let metadata = String::from_str(&env, "{\"gender\":\"M\",\"blood_type\":\"O+\"}");
        
        // Register patient
        client.register_patient(&user, &name, &dob, &metadata);
        
        // Update metadata
        let new_metadata = String::from_str(&env, "{\"gender\":\"M\",\"blood_type\":\"O+\",\"allergies\":[\"penicillin\"]}");
        client.update_patient(&user, &new_metadata);
        
        // Verify update
        let patient_data = client.get_patient(&user).unwrap();
        assert_eq!(patient_data.metadata, new_metadata);
    }
}