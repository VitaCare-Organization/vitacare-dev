#[cfg(test)]
mod tests {
    use soroban_sdk::{testutils::Address as _, vec, Address, Env, String};
    
    use crate::{PatientIdentityContract, PatientIdentityContractClient};
    use crate::types::PatientData;

    fn setup() -> (Env, Address, PatientIdentityContractClient) {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        
        // Initialize the contract
        client.initialize(&admin);
        
        (env, admin, client)
    }

    #[test]
    fn test_initialization() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PatientIdentityContract);
        let client = PatientIdentityContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        
        // Initialize contract
        client.initialize(&admin);
        
        // Verify all patients list is empty
        let patients = client.get_all_patients();
        assert_eq!(patients.len(), 0);
        
        // Try to initialize again, should panic
        let result = std::panic::catch_unwind(|| {
            client.initialize(&admin);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_patient_registration() {
        let (env, _admin, client) = setup();
        
        // Create test patient data
        let patient_wallet = Address::generate(&env);
        let name = String::from_slice(&env, "John Doe");
        let dob = 946684800; // 2000-01-01
        let metadata = String::from_slice(&env, "{\"gender\":\"male\",\"blood_type\":\"A+\"}");
        
        // Register the patient
        let patient_data = client.register_patient(&patient_wallet, &name, &dob, &metadata);
        
        // Verify registration was successful
        assert_eq!(patient_data.name, name);
        assert_eq!(patient_data.dob, dob);
        assert_eq!(patient_data.metadata, metadata);
        assert!(patient_data.registered_at > 0);
        assert_eq!(patient_data.registered_at, patient_data.last_updated);
        
        // Check that patient is in the list of all patients
        let all_patients = client.get_all_patients();
        assert_eq!(all_patients.len(), 1);
        assert_eq!(all_patients.get(0).unwrap(), patient_wallet);
        
        // Try to register the same patient again, should panic
        let result = std::panic::catch_unwind(|| {
            client.register_patient(&patient_wallet, &name, &dob, &metadata);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_patient_retrieval() {
        let (env, _admin, client) = setup();
        
        // Create and register a patient
        let patient_wallet = Address::generate(&env);
        let name = String::from_slice(&env, "Jane Smith");
        let dob = 978307200; // 2001-01-01
        let metadata = String::from_slice(&env, "{\"gender\":\"female\",\"blood_type\":\"O-\"}");
        
        client.register_patient(&patient_wallet, &name, &dob, &metadata);
        
        // Retrieve patient data
        let retrieved_data = client.get_patient(&patient_wallet);
        
        // Verify the retrieved data
        assert_eq!(retrieved_data.name, name);
        assert_eq!(retrieved_data.dob, dob);
        assert_eq!(retrieved_data.metadata, metadata);
        
        // Try to retrieve non-existent patient, should panic
        let non_existent_wallet = Address::generate(&env);
        let result = std::panic::catch_unwind(|| {
            client.get_patient(&non_existent_wallet);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_patient_update() {
        let (env, _admin, client) = setup();
        
        // Create and register a patient
        let patient_wallet = Address::generate(&env);
        let name = String::from_slice(&env, "Bob Johnson");
        let dob = 1009843200; // 2002-01-01
        let metadata = String::from_slice(&env, "{\"gender\":\"male\",\"blood_type\":\"B+\"}");
        
        let original_data = client.register_patient(&patient_wallet, &name, &dob, &metadata);
        
        // Update patient's metadata
        let updated_metadata = String::from_slice(&env, "{\"gender\":\"male\",\"blood_type\":\"B+\",\"allergies\":[\"peanuts\"]}");
        let updated_data = client.update_patient(&patient_wallet, &updated_metadata);
        
        // Verify update was successful
        assert_eq!(updated_data.name, name); // Name should be unchanged
        assert_eq!(updated_data.dob, dob); // DOB should be unchanged
        assert_eq!(updated_data.metadata, updated_metadata);
        assert_eq!(updated_data.registered_at, original_data.registered_at);
        assert!(updated_data.last_updated > original_data.last_updated);
        
        // Retrieve the updated data to verify storage
        let retrieved_data = client.get_patient(&patient_wallet);
        assert_eq!(retrieved_data.metadata, updated_metadata);
    }

    #[test]
    fn test_multiple_patients() {
        let (env, _admin, client) = setup();
        
        // Register 3 patients
        let patient1 = Address::generate(&env);
        let patient2 = Address::generate(&env);
        let patient3 = Address::generate(&env);
        
        client.register_patient(
            &patient1,
            &String::from_slice(&env, "Patient One"),
            &946684800,
            &String::from_slice(&env, "{\"note\":\"patient 1\"}")
        );
        
        client.register_patient(
            &patient2,
            &String::from_slice(&env, "Patient Two"),
            &978307200,
            &String::from_slice(&env, "{\"note\":\"patient 2\"}")
        );
        
        client.register_patient(
            &patient3,
            &String::from_slice(&env, "Patient Three"),
            &1009843200,
            &String::from_slice(&env, "{\"note\":\"patient 3\"}")
        );
        
        // Verify all patients list contains all three patients
        let all_patients = client.get_all_patients();
        assert_eq!(all_patients.len(), 3);
        
        // Check that each patient has the correct data
        let data1 = client.get_patient(&patient1);
        let data2 = client.get_patient(&patient2);
        let data3 = client.get_patient(&patient3);
        
        assert_eq!(data1.name.to_string(), "Patient One");
        assert_eq!(data2.name.to_string(), "Patient Two");
        assert_eq!(data3.name.to_string(), "Patient Three");
    }

    #[test]
    fn test_insurance_reference() {
        let (env, _admin, client) = setup();
        
        // Register a patient
        let patient_wallet = Address::generate(&env);
        client.register_patient(
            &patient_wallet,
            &String::from_slice(&env, "Insurance Test Patient"),
            &946684800,
            &String::from_slice(&env, "{\"gender\":\"female\"}")
        );
        
        // Add insurance reference
        let insurance_id = String::from_slice(&env, "INS123");
        let policy_number = String::from_slice(&env, "POL456");
        
        let updated_data = client.add_insurance_reference(
            &patient_wallet,
            &insurance_id,
            &policy_number
        );
        
        // Verify insurance info was added to metadata
        let metadata_str = updated_data.metadata.to_string();
        assert!(metadata_str.contains("INSURANCE"));
        assert!(metadata_str.contains("INS123"));
        assert!(metadata_str.contains("POL456"));
        
        // Add a second insurance reference
        let second_insurance_id = String::from_slice(&env, "INS789");
        let second_policy = String::from_slice(&env, "POL101112");
        
        let twice_updated_data = client.add_insurance_reference(
            &patient_wallet,
            &second_insurance_id,
            &second_policy
        );
        
        // Verify both insurance infos are in metadata
        let new_metadata_str = twice_updated_data.metadata.to_string();
        assert!(new_metadata_str.contains("INS123"));
        assert!(new_metadata_str.contains("INS789"));
    }

    #[test]
    fn test_medical_history_reference() {
        let (env, _admin, client) = setup();
        
        // Register a patient
        let patient_wallet = Address::generate(&env);
        client.register_patient(
            &patient_wallet,
            &String::from_slice(&env, "Medical History Test Patient"),
            &946684800,
            &String::from_slice(&env, "{\"gender\":\"male\"}")
        );
        
        // Add medical history reference
        let reference_id = String::from_slice(&env, "MED123");
        let reference_type = String::from_slice(&env, "ALLERGY_TEST");
        
        let updated_data = client.add_medical_history_reference(
            &patient_wallet,
            &reference_id,
            &reference_type
        );
        
        // Verify medical history was added
        let metadata_str = updated_data.metadata.to_string();
        assert!(metadata_str.contains("MEDICAL_HISTORY"));
        assert!(metadata_str.contains("MED123"));
        assert!(metadata_str.contains("ALLERGY_TEST"));
        
        // Add another medical record
        let second_ref_id = String::from_slice(&env, "MED456");
        let second_ref_type = String::from_slice(&env, "BLOOD_TEST");
        
        let twice_updated_data = client.add_medical_history_reference(
            &patient_wallet,
            &second_ref_id,
            &second_ref_type
        );
        
        // Verify both records are in metadata
        let new_metadata_str = twice_updated_data.metadata.to_string();
        assert!(new_metadata_str.contains("MED123"));
        assert!(new_metadata_str.contains("MED456"));
        assert!(new_metadata_str.contains("BLOOD_TEST"));
    }

    #[test]
    fn test_combined_references() {
        let (env, _admin, client) = setup();
        
        // Register a patient
        let patient_wallet = Address::generate(&env);
        client.register_patient(
            &patient_wallet,
            &String::from_slice(&env, "Combined Test Patient"),
            &946684800,
            &String::from_slice(&env, "{\"gender\":\"other\"}")
        );
        
        // Add both insurance and medical history
        client.add_insurance_reference(
            &patient_wallet,
            &String::from_slice(&env, "INS-ABC"),
            &String::from_slice(&env, "POL-XYZ")
        );
        
        let final_data = client.add_medical_history_reference(
            &patient_wallet,
            &String::from_slice(&env, "MED-987"),
            &String::from_slice(&env, "SURGERY")
        );
        
        // Verify both types of references exist
        let metadata_str = final_data.metadata.to_string();
        assert!(metadata_str.contains("INSURANCE"));
        assert!(metadata_str.contains("INS-ABC"));
        assert!(metadata_str.contains("MEDICAL_HISTORY"));
        assert!(metadata_str.contains("MED-987"));
        assert!(metadata_str.contains("SURGERY"));
    }
}