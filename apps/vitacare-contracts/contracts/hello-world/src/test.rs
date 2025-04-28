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
        assert_eq!(patient_data.insurance_ref, None);
        assert_eq!(patient_data.medical_history_ref, None);
        
        // Verify timestamp was set
        assert!(patient_data.last_updated > 0);
    }
    
    #[test]
    fn test_patient_duplicate_registration() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        
        let user = Address::generate(&env);
        let name = String::from_str(&env, "John Doe");
        let dob = 631152000;
        let metadata = String::from_str(&env, "{\"gender\":\"M\"}");
        
        // First registration should succeed
        client.register_patient(&user, &name, &dob, &metadata);
        
        // Second registration should fail
        let result = client.try_register_patient(&user, &name, &dob, &metadata);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().unwrap(), String::from_str(&env, "Patient already registered"));
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
        
        // Get initial timestamp
        let initial_data = client.get_patient(&user).unwrap();
        let initial_timestamp = initial_data.last_updated;
        
        // Advance ledger time
        env.ledger().with_mut(|li| {
            li.timestamp = initial_timestamp + 1000;
        });
        
        // Update metadata
        let new_metadata = String::from_str(&env, "{\"gender\":\"M\",\"blood_type\":\"O+\",\"allergies\":[\"penicillin\"]}");
        client.update_patient(&user, &new_metadata);
        
        // Verify update
        let patient_data = client.get_patient(&user).unwrap();
        assert_eq!(patient_data.metadata, new_metadata);
        assert_eq!(patient_data.name, name); // Name should remain unchanged
        assert_eq!(patient_data.dob, dob); // DOB should remain unchanged
        assert!(patient_data.last_updated > initial_timestamp); // Timestamp should be updated
    }
    
    #[test]
    fn test_update_nonexistent_patient() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        
        let user = Address::generate(&env);
        let metadata = String::from_str(&env, "{\"gender\":\"M\"}");
        
        // Update should fail for nonexistent patient
        let result = client.try_update_patient(&user, &metadata);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().unwrap(), String::from_str(&env, "Patient not found"));
    }
    
    #[test]
    fn test_get_nonexistent_patient() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        
        let user = Address::generate(&env);
        
        // Get should fail for nonexistent patient
        let result = client.try_get_patient(&user);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().unwrap(), String::from_str(&env, "Patient not found"));
    }
    
    #[test]
    fn test_add_insurance() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        
        let user = Address::generate(&env);
        let name = String::from_str(&env, "John Doe");
        let dob = 631152000;
        let metadata = String::from_str(&env, "{\"gender\":\"M\"}");
        
        // Register patient
        client.register_patient(&user, &name, &dob, &metadata);
        
        // Get initial timestamp
        let initial_data = client.get_patient(&user).unwrap();
        let initial_timestamp = initial_data.last_updated;
        
        // Advance ledger time
        env.ledger().with_mut(|li| {
            li.timestamp = initial_timestamp + 1000;
        });
        
        // Add insurance reference
        let insurance_ref = String::from_str(&env, "ipfs://QmInsuranceRecord123456");
        client.add_insurance(&user, &insurance_ref);
        
        // Verify insurance reference
        let patient_data = client.get_patient(&user).unwrap();
        assert_eq!(patient_data.insurance_ref, Some(insurance_ref));
        assert!(patient_data.last_updated > initial_timestamp); // Timestamp should be updated
    }
    
    #[test]
    fn test_add_insurance_nonexistent_patient() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        
        let user = Address::generate(&env);
        let insurance_ref = String::from_str(&env, "ipfs://QmInsuranceRecord123456");
        
        // Add insurance should fail for nonexistent patient
        let result = client.try_add_insurance(&user, &insurance_ref);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().unwrap(), String::from_str(&env, "Patient not found"));
    }
    
    #[test]
    fn test_add_medical_history() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        
        let user = Address::generate(&env);
        let name = String::from_str(&env, "Jane Doe");
        let dob = 694224000; // Different timestamp
        let metadata = String::from_str(&env, "{\"gender\":\"F\"}");
        
        // Register patient
        client.register_patient(&user, &name, &dob, &metadata);
        
        // Get initial timestamp
        let initial_data = client.get_patient(&user).unwrap();
        let initial_timestamp = initial_data.last_updated;
        
        // Advance ledger time
        env.ledger().with_mut(|li| {
            li.timestamp = initial_timestamp + 1000;
        });
        
        // Add medical history reference
        let medical_history_ref = String::from_str(&env, "ipfs://QmMedicalHistory789");
        client.add_medical_history(&user, &medical_history_ref);
        
        // Verify medical history reference
        let patient_data = client.get_patient(&user).unwrap();
        assert_eq!(patient_data.medical_history_ref, Some(medical_history_ref));
        assert!(patient_data.last_updated > initial_timestamp); // Timestamp should be updated
    }
    
    #[test]
    fn test_add_medical_history_nonexistent_patient() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        
        let user = Address::generate(&env);
        let medical_history_ref = String::from_str(&env, "ipfs://QmMedicalHistory789");
        
        // Add medical history should fail for nonexistent patient
        let result = client.try_add_medical_history(&user, &medical_history_ref);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().unwrap(), String::from_str(&env, "Patient not found"));
    }
    
    #[test]
    fn test_grant_access() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        
        let patient = Address::generate(&env);
        let provider = Address::generate(&env);
        let current_time = env.ledger().timestamp();
        let expiration = current_time + 86400; // 1 day access
        
        // Grant access
        client.grant_access(&patient, &provider, &expiration);
        
        // Check access - should be valid
        let has_access = client.check_access(&patient, &provider);
        assert!(has_access);
    }
    
    #[test]
    fn test_access_expiration() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        
        let patient = Address::generate(&env);
        let provider = Address::generate(&env);
        let current_time = env.ledger().timestamp();
        let expiration = current_time + 86400; // 1 day access
        
        // Grant access
        client.grant_access(&patient, &provider, &expiration);
        
        // Check access - should be valid
        let has_access = client.check_access(&patient, &provider);
        assert!(has_access);
        
        // Advance time beyond expiration
        env.ledger().with_mut(|li| {
            li.timestamp = expiration + 1;
        });
        
        // Check access again - should be expired
        let has_access = client.check_access(&patient, &provider);
        assert!(!has_access);
    }
    
    #[test]
    fn test_check_access_no_grant() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        
        let patient = Address::generate(&env);
        let provider = Address::generate(&env);
        
        // Check access without any grants
        let has_access = client.check_access(&patient, &provider);
        assert!(!has_access);
    }
    
    #[test]
    fn test_comprehensive_patient_workflow() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        
        // Create patient and provider addresses
        let patient = Address::generate(&env);
        let provider = Address::generate(&env);
        
        // Initial patient registration
        let name = String::from_str(&env, "Alice Johnson");
        let dob = 757382400; // Unix timestamp
        let metadata = String::from_str(&env, "{\"gender\":\"F\",\"height\":\"165cm\",\"weight\":\"60kg\"}");
        client.register_patient(&patient, &name, &dob, &metadata);
        
        // Update patient data
        let updated_metadata = String::from_str(&env, "{\"gender\":\"F\",\"height\":\"165cm\",\"weight\":\"62kg\",\"blood_pressure\":\"120/80\"}");
        client.update_patient(&patient, &updated_metadata);
        
        // Add insurance
        let insurance_ref = String::from_str(&env, "ipfs://QmInsuranceDataXYZ");
        client.add_insurance(&patient, &insurance_ref);
        
        // Add medical history
        let medical_history_ref = String::from_str(&env, "ipfs://QmMedicalHistoryABC");
        client.add_medical_history(&patient, &medical_history_ref);
        
        // Grant access to provider
        let current_time = env.ledger().timestamp();
        let expiration = current_time + 604800; // 1 week access
        client.grant_access(&patient, &provider, &expiration);
        
        // Verify all data
        let patient_data = client.get_patient(&patient).unwrap();
        assert_eq!(patient_data.name, name);
        assert_eq!(patient_data.dob, dob);
        assert_eq!(patient_data.metadata, updated_metadata);
        assert_eq!(patient_data.insurance_ref, Some(insurance_ref));
        assert_eq!(patient_data.medical_history_ref, Some(medical_history_ref));
        
        // Verify access
        let has_access = client.check_access(&patient, &provider);
        assert!(has_access);
        
        // Advance time to just before expiration
        env.ledger().with_mut(|li| {
            li.timestamp = expiration - 1;
        });
        
        // Access should still be valid
        let has_access = client.check_access(&patient, &provider);
        assert!(has_access);
        
        // Advance time to after expiration
        env.ledger().with_mut(|li| {
            li.timestamp = expiration + 1;
        });
        
        // Access should be expired
        let has_access = client.check_access(&patient, &provider);
        assert!(!has_access);
    }
}
