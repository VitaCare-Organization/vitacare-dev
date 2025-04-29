#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod patient_identity {
    use ink_storage::{
        collections::HashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    /// Defines the patient data structure
    #[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct PatientData {
        /// Patient's full name
        name: String,
        /// Date of birth stored as Unix timestamp
        dob: u64,
        /// Additional metadata in JSON format
        /// Can include insurance info, medical history references, etc.
        metadata: String,
        /// Timestamp of when the patient was registered
        registered_at: u64,
        /// Timestamp of the last update
        last_updated_at: u64,
    }

    /// The contract storage
    #[ink(storage)]
    pub struct PatientIdentity {
        /// Mapping from patient wallet address to patient data
        patients: HashMap<AccountId, PatientData>,
        /// Contract owner
        owner: AccountId,
    }

    /// Events emitted by the contract
    #[ink(event)]
    pub struct PatientRegistered {
        #[ink(topic)]
        patient: AccountId,
        name: String,
        timestamp: u64,
    }

    #[ink(event)]
    pub struct PatientUpdated {
        #[ink(topic)]
        patient: AccountId,
        timestamp: u64,
    }

    /// Errors that can occur in the contract
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Patient already exists
        PatientAlreadyExists,
        /// Patient does not exist
        PatientNotFound,
        /// Caller is not authorized
        NotAuthorized,
        /// Invalid input data
        InvalidInput,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl PatientIdentity {
        /// Constructor to initialize the contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                patients: HashMap::new(),
                owner: Self::env().caller(),
            }
        }

        /// Register a new patient with the provided details
        #[ink(message)]
        pub fn register_patient(&mut self, wallet: AccountId, name: String, dob: u64, metadata: String) -> Result<()> {
            // Validate inputs
            if name.is_empty() {
                return Err(Error::InvalidInput);
            }

            // Check if patient already exists
            if self.patients.contains_key(&wallet) {
                return Err(Error::PatientAlreadyExists);
            }

            let now = self.env().block_timestamp();
            
            // Create new patient record
            let patient = PatientData {
                name,
                dob,
                metadata,
                registered_at: now,
                last_updated_at: now,
            };

            // Store the patient data
            self.patients.insert(wallet, patient.clone());

            // Emit event
            self.env().emit_event(PatientRegistered {
                patient: wallet,
                name: patient.name,
                timestamp: now,
            });

            Ok(())
        }

        /// Update an existing patient's metadata
        #[ink(message)]
        pub fn update_patient(&mut self, wallet: AccountId, metadata: String) -> Result<()> {
            // Check if the patient exists
            let patient = self.patients.get_mut(&wallet).ok_or(Error::PatientNotFound)?;
            
            // Check authorization - only the patient or the contract owner can update
            let caller = self.env().caller();
            if caller != wallet && caller != self.owner {
                return Err(Error::NotAuthorized);
            }

            // Update patient metadata
            patient.metadata = metadata;
            patient.last_updated_at = self.env().block_timestamp();

            // Emit event
            self.env().emit_event(PatientUpdated {
                patient: wallet,
                timestamp: patient.last_updated_at,
            });

            Ok(())
        }

        /// Get patient data for a specific wallet
        #[ink(message)]
        pub fn get_patient(&self, wallet: AccountId) -> Result<PatientData> {
            // Return patient data if exists
            self.patients.get(&wallet).cloned().ok_or(Error::PatientNotFound)
        }

        /// Check if a patient exists
        #[ink(message)]
        pub fn patient_exists(&self, wallet: AccountId) -> bool {
            self.patients.contains_key(&wallet)
        }

        /// Get the total number of registered patients
        #[ink(message)]
        pub fn get_patient_count(&self) -> u64 {
            self.patients.len() as u64
        }
    }

    /// Unit tests for the contract
    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn register_patient_works() {
            let mut contract = PatientIdentity::new();
            let account_id = AccountId::from([0x1; 32]);
            
            // Register a new patient
            let result = contract.register_patient(
                account_id,
                String::from("John Doe"),
                1609459200, // 2021-01-01 00:00:00
                String::from("{\"insurance\":\"ABC123\",\"medical_history\":\"xyz.com/records\"}"),
            );
            
            assert!(result.is_ok());
            assert!(contract.patient_exists(account_id));
        }

        #[ink::test]
        fn duplicate_registration_fails() {
            let mut contract = PatientIdentity::new();
            let account_id = AccountId::from([0x1; 32]);
            
            // Register a patient
            let _ = contract.register_patient(
                account_id,
                String::from("John Doe"),
                1609459200,
                String::from("{}"),
            );
            
            // Try to register the same patient again
            let result = contract.register_patient(
                account_id,
                String::from("John Doe"),
                1609459200,
                String::from("{}"),
            );
            
            assert_eq!(result, Err(Error::PatientAlreadyExists));
        }

        #[ink::test]
        fn update_patient_works() {
            let mut contract = PatientIdentity::new();
            let account_id = AccountId::from([0x1; 32]);
            
            // Register a patient
            let _ = contract.register_patient(
                account_id,
                String::from("John Doe"),
                1609459200,
                String::from("{\"insurance\":\"ABC123\"}"),
            );
            
            // Update the patient
            let result = contract.update_patient(
                account_id,
                String::from("{\"insurance\":\"XYZ789\",\"medical_history\":\"xyz.com/records\"}"),
            );
            
            assert!(result.is_ok());
            
            // Check the updated data
            let patient = contract.get_patient(account_id).unwrap();
            assert_eq!(patient.metadata, String::from("{\"insurance\":\"XYZ789\",\"medical_history\":\"xyz.com/records\"}"));
        }

        #[ink::test]
        fn get_patient_works() {
            let mut contract = PatientIdentity::new();
            let account_id = AccountId::from([0x1; 32]);
            
            // Register a patient
            let _ = contract.register_patient(
                account_id,
                String::from("John Doe"),
                1609459200,
                String::from("{\"insurance\":\"ABC123\"}"),
            );
            
            // Get the patient data
            let patient = contract.get_patient(account_id);
            
            assert!(patient.is_ok());
            let patient_data = patient.unwrap();
            assert_eq!(patient_data.name, String::from("John Doe"));
            assert_eq!(patient_data.dob, 1609459200);
        }

        #[ink::test]
        fn get_nonexistent_patient_fails() {
            let contract = PatientIdentity::new();
            let account_id = AccountId::from([0x1; 32]);
            
            // Try to get a non-existent patient
            let result = contract.get_patient(account_id);
            
            assert_eq!(result, Err(Error::PatientNotFound));
        }
    }
}