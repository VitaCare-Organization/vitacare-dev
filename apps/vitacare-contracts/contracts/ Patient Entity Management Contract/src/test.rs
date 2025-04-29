#![cfg(test)]
mod tests {
    use super::*;
    use ink_lang as ink;
    use ink_env::{
        test,
        AccountId,
        DefaultEnvironment,
        Environment,
    };
    
    // Helper function to create a test account
    fn create_account(id: u8) -> AccountId {
        let mut account = [0u8; 32];
        account[0] = id;
        AccountId::from(account)
    }
    
    // Helper function to create JSON metadata
    fn create_metadata(insurance: Option<&str>, medical_history: Option<&str>) -> String {
        let mut json = String::from("{");
        
        let mut has_prev_field = false;
        
        if let Some(ins) = insurance {
            json.push_str(&format!("\"insurance\":\"{ins}\""));
            has_prev_field = true;
        }
        
        if let Some(hist) = medical_history {
            if has_prev_field {
                json.push_str(",");
            }
            json.push_str(&format!("\"medical_history\":\"{hist}\""));
        }
        
        json.push_str("}");
        json
    }

    fn create_default_contract() -> PatientIdentity {
        let accounts = ink_env::test::default_accounts::<DefaultEnvironment>();
        let contract_id = accounts.alice;
        
        // Set the caller to Alice (contract creator)
        ink_env::test::set_caller::<DefaultEnvironment>(contract_id);
        
        PatientIdentity::new()
    }
    
    fn register_test_patient(contract: &mut PatientIdentity, patient_id: AccountId, name: &str) -> Result<()> {
        let dob = 946684800; // 2000-01-01
        let metadata = create_metadata(
            Some("INS123456"),
            Some("https://medicalrecords.example/patient123"),
        );
        
        contract.register_patient(patient_id, String::from(name), dob, metadata)
    }
    
    #[ink::test]
    fn constructor_works() {
        let contract = create_default_contract();
        assert_eq!(contract.get_patient_count(), 0);
    }
    
    #[ink::test]
    fn registration_works() {
        let mut contract = create_default_contract();
        let patient = create_account(1);
        
        // Register a patient
        let result = register_test_patient(&mut contract, patient, "Alice Patient");
        assert!(result.is_ok());
        
        // Verify registration
        assert!(contract.patient_exists(patient));
        assert_eq!(contract.get_patient_count(), 1);
        
        // Check patient data
        let patient_data = contract.get_patient(patient).unwrap();
        assert_eq!(patient_data.name, "Alice Patient");
        assert_eq!(patient_data.dob, 946684800);
        assert!(patient_data.metadata.contains("INS123456"));
        assert!(patient_data.metadata.contains("medicalrecords.example"));
    }
    
    #[ink::test]
    fn registration_with_empty_name_fails() {
        let mut contract = create_default_contract();
        let patient = create_account(1);
        
        // Try to register with empty name
        let result = contract.register_patient(
            patient,
            String::from(""),  // Empty name
            946684800,
            create_metadata(Some("INS123"), None),
        );
        
        assert_eq!(result, Err(Error::InvalidInput));
        assert!(!contract.patient_exists(patient));
    }
    
    #[ink::test]
    fn duplicate_registration_fails() {
        let mut contract = create_default_contract();
        let patient = create_account(1);
        
        // Register a patient first
        let _ = register_test_patient(&mut contract, patient, "Alice Patient");
        
        // Try to register the same patient again
        let result = register_test_patient(&mut contract, patient, "Alice Again");
        
        assert_eq!(result, Err(Error::PatientAlreadyExists));
        assert_eq!(contract.get_patient_count(), 1);
    }
    
    #[ink::test]
    fn update_patient_works() {
        let mut contract = create_default_contract();
        let contract_owner = ink_env::test::default_accounts::<DefaultEnvironment>().alice;
        let patient = create_account(1);
        
        // Register a patient
        let _ = register_test_patient(&mut contract, patient, "Alice Patient");
        
        // Set caller to the patient account
        ink_env::test::set_caller::<DefaultEnvironment>(patient);
        
        // Update patient metadata
        let new_metadata = create_metadata(
            Some("NEW-INS789"),
            Some("https://newrecords.example/alice"),
        );
        
        let result = contract.update_patient(patient, new_metadata.clone());
        assert!(result.is_ok());
        
        // Check updated data
        let patient_data = contract.get_patient(patient).unwrap();
        assert_eq!(patient_data.metadata, new_metadata);
        assert_eq!(patient_data.name, "Alice Patient"); // Name shouldn't change
    }
    
    #[ink::test]
    fn owner_can_update_any_patient() {
        let mut contract = create_default_contract();
        let contract_owner = ink_env::test::default_accounts::<DefaultEnvironment>().alice;
        let patient = create_account(1);
        
        // Register a patient
        let _ = register_test_patient(&mut contract, patient, "Alice Patient");
        
        // Set caller to contract owner
        ink_env::test::set_caller::<DefaultEnvironment>(contract_owner);
        
        // Owner updates patient metadata
        let new_metadata = create_metadata(
            Some("OWNER-UPDATE-INS"),
            Some("https://admin-updated.example"),
        );
        
        let result = contract.update_patient(patient, new_metadata.clone());
        assert!(result.is_ok());
        
        // Check updated data
        let patient_data = contract.get_patient(patient).unwrap();
        assert_eq!(patient_data.metadata, new_metadata);
    }
    
    #[ink::test]
    fn unauthorized_update_fails() {
        let mut contract = create_default_contract();
        let patient = create_account(1);
        let unauthorized_user = create_account(2);
        
        // Register a patient
        let _ = register_test_patient(&mut contract, patient, "Alice Patient");
        
        // Set caller to unauthorized user
        ink_env::test::set_caller::<DefaultEnvironment>(unauthorized_user);
        
        // Try to update patient metadata
        let result = contract.update_patient(
            patient,
            create_metadata(Some("HACKED"), Some("BAD-DATA")),
        );
        
        assert_eq!(result, Err(Error::NotAuthorized));
        
        // Verify data wasn't changed
        let patient_data = contract.get_patient(patient).unwrap();
        assert!(patient_data.metadata.contains("INS123456"));
    }
    
    #[ink::test]
    fn get_non_existent_patient_fails() {
        let contract = create_default_contract();
        let non_existent = create_account(99);
        
        let result = contract.get_patient(non_existent);
        assert_eq!(result, Err(Error::PatientNotFound));
    }
    
    #[ink::test]
    fn update_non_existent_patient_fails() {
        let mut contract = create_default_contract();
        let non_existent = create_account(99);
        
        let result = contract.update_patient(
            non_existent, 
            create_metadata(Some("INSURANCE"), None),
        );
        
        assert_eq!(result, Err(Error::PatientNotFound));
    }
    
    #[ink::test]
    fn multiple_patients_work() {
        let mut contract = create_default_contract();
        
        // Create multiple patients
        let total_patients = 5;
        for i in 1..=total_patients {
            let patient = create_account(i as u8);
            let name = format!("Patient {}", i);
            let _ = register_test_patient(&mut contract, patient, &name);
        }
        
        // Verify count
        assert_eq!(contract.get_patient_count(), total_patients);
        
        // Verify each patient exists
        for i in 1..=total_patients {
            let patient = create_account(i as u8);
            assert!(contract.patient_exists(patient));
            
            let patient_data = contract.get_patient(patient).unwrap();
            assert_eq!(patient_data.name, format!("Patient {}", i));
        }
    }
    
    #[ink::test]
    fn timestamps_are_set() {
        let mut contract = create_default_contract();
        let patient = create_account(1);
        
        // Set a mock block timestamp
        let registration_time = 1000;
        ink_env::test::set_block_timestamp::<DefaultEnvironment>(registration_time);
        
        // Register patient
        let _ = register_test_patient(&mut contract, patient, "Timestamp Test");
        
        // Check registration timestamp
        let patient_data = contract.get_patient(patient).unwrap();
        assert_eq!(patient_data.registered_at, registration_time);
        assert_eq!(patient_data.last_updated_at, registration_time);
        
        // Set a new timestamp and update
        let update_time = 2000;
        ink_env::test::set_block_timestamp::<DefaultEnvironment>(update_time);
        
        // Set caller to patient for update
        ink_env::test::set_caller::<DefaultEnvironment>(patient);
        
        // Update patient
        let _ = contract.update_patient(
            patient,
            create_metadata(Some("UPDATED-INS"), None),
        );
        
        // Check updated timestamp
        let updated_data = contract.get_patient(patient).unwrap();
        assert_eq!(updated_data.registered_at, registration_time); // Should not change
        assert_eq!(updated_data.last_updated_at, update_time); // Should be updated
    }
    
    #[ink::test]
    fn events_are_emitted() {
        let mut contract = create_default_contract();
        let patient = create_account(1);
        
        // Clear previous events
        ink_env::test::recorded_events().drain();
        
        // Register a patient and record events
        let _ = register_test_patient(&mut contract, patient, "Event Test");
        
        // Check if registration event was emitted
        let recorded_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(recorded_events.len(), 1);
        
        // Update and check for update event
        ink_env::test::recorded_events().drain();
        ink_env::test::set_caller::<DefaultEnvironment>(patient);
        let _ = contract.update_patient(
            patient,
            create_metadata(Some("EVENT-TEST"), None),
        );
        
        let update_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(update_events.len(), 1);
    }
    
    #[ink::test]
    fn complex_metadata_works() {
        let mut contract = create_default_contract();
        let patient = create_account(1);
        
        // Create complex JSON metadata
        let complex_metadata = r#"{"insurance":{"provider":"HealthCorp","policy_number":"POL123456","expiry":"2025-01-01"},"medical_history":{"allergies":["penicillin","peanuts"],"conditions":["asthma"],"blood_type":"O+"},"emergency_contact":{"name":"John Doe","relationship":"Spouse","phone":"+1234567890"}}"#.to_string();
        
        // Register with complex metadata
        let result = contract.register_patient(
            patient,
            String::from("Complex Patient"),
            946684800,
            complex_metadata.clone(),
        );
        
        assert!(result.is_ok());
        
        // Check metadata is stored correctly
        let patient_data = contract.get_patient(patient).unwrap();
        assert_eq!(patient_data.metadata, complex_metadata);
    }
    
    #[ink::test]
    fn very_large_patient_batch() {
        let mut contract = create_default_contract();
        
        // Register a large number of patients to test capacity
        let large_batch = 100; // Adjust based on test environment capabilities
        
        for i in 1..=large_batch {
            let patient = create_account(i as u8);
            let metadata = create_metadata(
                Some(&format!("INS{}", i)),
                Some(&format!("HISTORY{}", i)),
            );
            
            let result = contract.register_patient(
                patient,
                format!("Batch Patient {}", i),
                946684800,
                metadata,
            );
            
            assert!(result.is_ok());
        }
        
        assert_eq!(contract.get_patient_count(), large_batch);
    }
}