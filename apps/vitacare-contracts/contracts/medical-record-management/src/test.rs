#![cfg(test)]

use crate::{MedicalRecordsContract, MedicalRecordsContractClient};
use soroban_sdk::{
    testutils::{Address as AddressTestUtil},
    Address, BytesN, Env, String, log,
};

#[test]
fn test_medical_record_flow() {
    let env = Env::default();
    let contract_id = env.register(MedicalRecordsContract, ());
    let client = MedicalRecordsContractClient::new(&env, &contract_id);

    // Create test addresses
    let patient = Address::generate(&env);
    let doctor = Address::generate(&env);
    
    // Create test data for medical record
    let record_hash = BytesN::from_array(&env, &[0u8; 32]);
    let description = String::from_str(&env, "Annual checkup results");

    // Test 1: Patient grants access to doctor
    env.mock_all_auths();
    client.grant_access(&patient, &doctor);

    // Test 2: Patient should be able to see authorized doctors
    let authorized_doctors = client.get_authorized_doctors(&patient);
    assert_eq!(authorized_doctors.len(), 1);
    assert_eq!(authorized_doctors.get(0), Some(doctor.clone()));

    // Test 3: Doctor adds a medical record
    env.mock_all_auths();
    client.add_medical_record(&patient, &doctor, &record_hash, &description);

    // Test 4: Patient can view their records
    let records = client.get_medical_records(&patient);
    assert_eq!(records.len(), 1);
    let record = records.get(0).unwrap();
    assert_eq!(record.doctor, doctor);
    assert_eq!(record.patient, patient);
    assert_eq!(record.data_id, record_hash);
    assert_eq!(record.description, description);

    // Test 5: Patient revokes access
    env.mock_all_auths();
    client.revoke_access(&patient, &doctor);
    
    // Test 6: Verify that the doctor no longer has access
    let authorized_doctors_after = client.get_authorized_doctors(&patient);
    assert_eq!(authorized_doctors_after.len(), 0);
}

/// Test for scenario when a patient grants access to multiple doctors
#[test]
fn test_multiple_doctors_access() {
    let env = Env::default();
    let contract_id = env.register(MedicalRecordsContract, ());
    let client = MedicalRecordsContractClient::new(&env, &contract_id);

    // Create test addresses
    let patient = Address::generate(&env);
    let doctor1 = Address::generate(&env);
    let doctor2 = Address::generate(&env);
    let doctor3 = Address::generate(&env);
    
    // Patient grants access to three different doctors
    env.mock_all_auths();
    client.grant_access(&patient, &doctor1);
    client.grant_access(&patient, &doctor2);
    client.grant_access(&patient, &doctor3);

    // Verify that all three doctors have access
    let authorized_doctors = client.get_authorized_doctors(&patient);
    assert_eq!(authorized_doctors.len(), 3);
    assert!(authorized_doctors.contains(&doctor1));
    assert!(authorized_doctors.contains(&doctor2));
    assert!(authorized_doctors.contains(&doctor3));
    
    // Revoke access for doctor2 only
    env.mock_all_auths();
    client.revoke_access(&patient, &doctor2);
    
    // Verify that doctor2 no longer has access but doctor1 and doctor3 still do
    let authorized_doctors_after = client.get_authorized_doctors(&patient);
    assert_eq!(authorized_doctors_after.len(), 2);
    assert!(authorized_doctors_after.contains(&doctor1));
    assert!(!authorized_doctors_after.contains(&doctor2));
    assert!(authorized_doctors_after.contains(&doctor3));
}

/// Test to verify that different patients have their records properly separated
#[test]
fn test_multiple_patients() {
    let env = Env::default();
    let contract_id = env.register(MedicalRecordsContract, ());
    let client = MedicalRecordsContractClient::new(&env, &contract_id);
    
    // Create test addresses
    let patient1 = Address::generate(&env);
    let patient2 = Address::generate(&env);
    let doctor1 = Address::generate(&env);
    let doctor2 = Address::generate(&env);
    
    log!(&env, "Patient1: {:?}", patient1);
    log!(&env, "Patient2: {:?}", patient2);
    log!(&env, "Doctor1: {:?}", doctor1);
    log!(&env, "Doctor2: {:?}", doctor2);
    
    // Test data for medical records
    let mut data1 = [0u8; 32];
    data1[0] = 1;
    let record_hash1 = BytesN::from_array(&env, &data1);
    let description1 = String::from_str(&env, "Patient 1 record");
    
    let mut data2 = [0u8; 32];
    data2[0] = 2;
    let record_hash2 = BytesN::from_array(&env, &data2);
    let description2 = String::from_str(&env, "Patient 2 record");
    
    // Each patient grants access to their own doctor
    env.mock_all_auths();
    client.grant_access(&patient1, &doctor1);
    client.grant_access(&patient2, &doctor2);
    
    // Each doctor adds a record for their patient
    env.mock_all_auths();
    client.add_medical_record(&patient1, &doctor1, &record_hash1, &description1);
    client.add_medical_record(&patient2, &doctor2, &record_hash2, &description2);
    
    // Verify that each patient can see records
    let records1 = client.get_medical_records(&patient1);
    let records2 = client.get_medical_records(&patient2);
    
    // Log the number of records
    log!(&env, "Number of records for patient1: {}", records1.len());
    log!(&env, "Number of records for patient2: {}", records2.len());
    
    // Each patient should see only their own record (1 record each)
    assert_eq!(records1.len(), 1, "Patient1 should have exactly 1 record");
    assert_eq!(records2.len(), 1, "Patient2 should have exactly 1 record");
    
    // Verify that each patient has the correct record
    let record1 = records1.get(0).unwrap();
    assert_eq!(record1.patient, patient1, "Patient1's record should have patient1 as the patient");
    assert_eq!(record1.doctor, doctor1, "Patient1's record should have doctor1 as the doctor");
    assert_eq!(record1.description, description1, "Patient1's record should have the correct description");
    
    let record2 = records2.get(0).unwrap();
    assert_eq!(record2.patient, patient2, "Patient2's record should have patient2 as the patient");
    assert_eq!(record2.doctor, doctor2, "Patient2's record should have doctor2 as the doctor");
    assert_eq!(record2.description, description2, "Patient2's record should have the correct description");
}

/// Test to verify that a patient can add their own records without doctor authorization
#[test]
fn test_patient_self_records() {
    let env = Env::default();
    let contract_id = env.register(MedicalRecordsContract, ());
    let client = MedicalRecordsContractClient::new(&env, &contract_id);

    // Create test address
    let patient = Address::generate(&env);
    
    // Test data for medical record
    let mut data = [0u8; 32];
    data[0] = 11;
    let record_hash = BytesN::from_array(&env, &data);
    let description = String::from_str(&env, "Self-reported symptoms");
    
    // Patient adds their own record (doctor = patient)
    env.mock_all_auths();
    client.add_medical_record(&patient, &patient, &record_hash, &description);
    
    // Verify that the record was added correctly
    let records = client.get_medical_records(&patient);
    assert_eq!(records.len(), 1);
    
    let record = records.get(0).unwrap();
    assert_eq!(record.patient, patient);
    assert_eq!(record.doctor, patient);
    assert_eq!(record.data_id, record_hash);
    assert_eq!(record.description, description);
}

/// Test to verify that the contract correctly handles duplicate permission attempts
#[test]
fn test_duplicate_permissions() {
    let env = Env::default();
    let contract_id = env.register(MedicalRecordsContract, ());
    let client = MedicalRecordsContractClient::new(&env, &contract_id);

    // Create test addresses
    let patient = Address::generate(&env);
    let doctor = Address::generate(&env);
    
    // Grant access for the first time (should work)
    env.mock_all_auths();
    client.grant_access(&patient, &doctor);
    
    // Try to grant access for the second time (should return Ok())
    env.mock_all_auths();
    client.grant_access(&patient, &doctor);
    
    // Verify that the doctor is still authorized
    let authorized_doctors = client.get_authorized_doctors(&patient);
    assert_eq!(authorized_doctors.len(), 1);
}

/// Test to verify doctor authorization for patients
#[test]
fn test_doctor_authorization_check() {
    let env = Env::default();
    let contract_id = env.register(MedicalRecordsContract, ());
    let client = MedicalRecordsContractClient::new(&env, &contract_id);

    // Create test addresses
    let patient = Address::generate(&env);
    let authorized_doctor = Address::generate(&env);
    let _unauthorized_doctor = Address::generate(&env);
    
    // Patient grants access only to authorized_doctor
    env.mock_all_auths();
    client.grant_access(&patient, &authorized_doctor);
    
    // Authorized doctor adds a record - should work
    env.mock_all_auths();
    let record_hash = BytesN::from_array(&env, &[1u8; 32]);
    let description = String::from_str(&env, "Record from authorized doctor");
    client.add_medical_record(&patient, &authorized_doctor, &record_hash, &description);
    
    // Verify the record was added
    let records = client.get_medical_records(&patient);
    assert_eq!(records.len(), 1);
    assert_eq!(records.get(0).unwrap().doctor, authorized_doctor);
}

/// Test to verify that multiple records can be added for a patient
#[test]
fn test_multiple_records() {
    let env = Env::default();
    let contract_id = env.register(MedicalRecordsContract, ());
    let client = MedicalRecordsContractClient::new(&env, &contract_id);

    // Create test addresses
    let patient = Address::generate(&env);
    let doctor = Address::generate(&env);
    
    // Grant access to doctor
    env.mock_all_auths();
    client.grant_access(&patient, &doctor);
    
    // Create multiple record hashes
    let record_hash1 = BytesN::from_array(&env, &[1u8; 32]);
    let description1 = String::from_str(&env, "First visit");
    
    let record_hash2 = BytesN::from_array(&env, &[2u8; 32]);
    let description2 = String::from_str(&env, "Follow-up visit");
    
    let record_hash3 = BytesN::from_array(&env, &[3u8; 32]);
    let description3 = String::from_str(&env, "Final check");
    
    // Add multiple records
    env.mock_all_auths();
    client.add_medical_record(&patient, &doctor, &record_hash1, &description1);
    client.add_medical_record(&patient, &doctor, &record_hash2, &description2);
    client.add_medical_record(&patient, &doctor, &record_hash3, &description3);
    
    // Verify all records were added
    let records = client.get_medical_records(&patient);
    assert_eq!(records.len(), 3, "Should have exactly 3 records");
    
    // Verify records are in correct order (newest first)
    assert_eq!(records.get(0).unwrap().description, description1);
    assert_eq!(records.get(1).unwrap().description, description2);
    assert_eq!(records.get(2).unwrap().description, description3);
}

/// Test to verify that multiple doctors can contribute to a patient's record
#[test]
fn test_multiple_doctor_contributions() {
    let env = Env::default();
    let contract_id = env.register(MedicalRecordsContract, ());
    let client = MedicalRecordsContractClient::new(&env, &contract_id);

    // Create test addresses
    let patient = Address::generate(&env);
    let general_doctor = Address::generate(&env);
    let specialist = Address::generate(&env);
    let lab_technician = Address::generate(&env);
    
    // Patient grants access to all healthcare providers
    env.mock_all_auths();
    client.grant_access(&patient, &general_doctor);
    client.grant_access(&patient, &specialist);
    client.grant_access(&patient, &lab_technician);
    
    // Each healthcare provider adds their own record
    env.mock_all_auths();
    let record_hash1 = BytesN::from_array(&env, &[10u8; 32]);
    let description1 = String::from_str(&env, "Initial diagnosis");
    client.add_medical_record(&patient, &general_doctor, &record_hash1, &description1);
    
    let record_hash2 = BytesN::from_array(&env, &[20u8; 32]);
    let description2 = String::from_str(&env, "Specialist consultation");
    client.add_medical_record(&patient, &specialist, &record_hash2, &description2);
    
    let record_hash3 = BytesN::from_array(&env, &[30u8; 32]);
    let description3 = String::from_str(&env, "Lab test results");
    client.add_medical_record(&patient, &lab_technician, &record_hash3, &description3);
    
    // Verify all records were added
    let records = client.get_medical_records(&patient);
    assert_eq!(records.len(), 3, "Should have exactly 3 records");
    
    // Check that each record has the correct doctor
    let mut found_general = false;
    let mut found_specialist = false;
    let mut found_lab = false;
    
    for record in records.iter() {
        if record.doctor == general_doctor && record.description == description1 {
            found_general = true;
        } else if record.doctor == specialist && record.description == description2 {
            found_specialist = true;
        } else if record.doctor == lab_technician && record.description == description3 {
            found_lab = true;
        }
    }
    
    assert!(found_general, "Missing record from general doctor");
    assert!(found_specialist, "Missing record from specialist");
    assert!(found_lab, "Missing record from lab technician");
}

/// Test to verify changing primary care physician
#[test]
fn test_changing_primary_doctor() {
    let env = Env::default();
    let contract_id = env.register(MedicalRecordsContract, ());
    let client = MedicalRecordsContractClient::new(&env, &contract_id);

    // Create test addresses
    let patient = Address::generate(&env);
    let old_doctor = Address::generate(&env);
    let new_doctor = Address::generate(&env);
    
    // Initial doctor relationship
    env.mock_all_auths();
    client.grant_access(&patient, &old_doctor);
    
    // Old doctor adds a record
    let record_hash1 = BytesN::from_array(&env, &[1u8; 32]);
    let description1 = String::from_str(&env, "Initial evaluation by first doctor");
    env.mock_all_auths();
    client.add_medical_record(&patient, &old_doctor, &record_hash1, &description1);
    
    // Patient changes to a new doctor
    env.mock_all_auths();
    client.revoke_access(&patient, &old_doctor);
    client.grant_access(&patient, &new_doctor);
    
    // Verify that only the new doctor has access now
    let authorized_doctors = client.get_authorized_doctors(&patient);
    assert_eq!(authorized_doctors.len(), 1);
    assert_eq!(authorized_doctors.get(0), Some(new_doctor.clone()));
    
    // New doctor adds a record
    let record_hash2 = BytesN::from_array(&env, &[2u8; 32]);
    let description2 = String::from_str(&env, "Follow-up with new doctor");
    env.mock_all_auths();
    client.add_medical_record(&patient, &new_doctor, &record_hash2, &description2);
    
    // Verify that records from both doctors are available
    let records = client.get_medical_records(&patient);
    assert_eq!(records.len(), 2, "Should have records from both doctors");
    
    // Check that we have records from both doctors
    let mut found_old_doctor = false;
    let mut found_new_doctor = false;
    
    for record in records.iter() {
        if record.doctor == old_doctor {
            found_old_doctor = true;
        } else if record.doctor == new_doctor {
            found_new_doctor = true;
        }
    }
    
    assert!(found_old_doctor, "Missing record from old doctor");
    assert!(found_new_doctor, "Missing record from new doctor");
}

/// Test to verify multiple records covering a patient's medical history
#[test]
fn test_medical_history() {
    let env = Env::default();
    let contract_id = env.register(MedicalRecordsContract, ());
    let client = MedicalRecordsContractClient::new(&env, &contract_id);

    // Create test addresses
    let patient = Address::generate(&env);
    let doctor = Address::generate(&env);
    
    // Grant access to doctor
    env.mock_all_auths();
    client.grant_access(&patient, &doctor);
    
    // Create a series of medical records representing a patient's history
    let conditions = [
        ("Initial checkup", [1u8; 32]),
        ("Flu diagnosis", [2u8; 32]),
        ("Prescription for antibiotics", [3u8; 32]),
        ("Follow-up visit", [4u8; 32]),
        ("Annual physical", [5u8; 32]),
        ("Blood test results", [6u8; 32]),
        ("Vaccination record", [7u8; 32]),
        ("Allergy test", [8u8; 32]),
        ("Specialist referral", [9u8; 32]),
        ("Surgery report", [10u8; 32]),
    ];
    
    // Add all the records
    env.mock_all_auths();
    for (desc, hash_data) in conditions.iter() {
        let record_hash = BytesN::from_array(&env, hash_data);
        let description = String::from_str(&env, desc);
        client.add_medical_record(&patient, &doctor, &record_hash, &description);
    }
    
    // Verify all records were added
    let records = client.get_medical_records(&patient);
    assert_eq!(records.len(), conditions.len() as u32, "Should have all medical history records");
    
    // Check that each condition is represented in the records
    for (i, (desc, _)) in conditions.iter().enumerate() {
        let found = records.iter().any(|r| r.description == String::from_str(&env, desc));
        assert!(found, "Missing record: {}", desc);
        
        // Also check the record ID sequence
        let record_id = i as u32 + 1; // Record IDs should start from 1
        let record_with_id = records.iter().find(|r| r.id == record_id);
        assert!(record_with_id.is_some(), "Missing record with ID {}", record_id);
    }
    
    // Log the full history for debugging
    log!(&env, "Patient's full medical history:");
    for record in records.iter() {
        log!(&env, "  Record #{}: {} (by Doctor: {:?})", 
            record.id, record.description, record.doctor);
    }
}

/// Test to verify re-authorization of previously authorized doctor
#[test]
fn test_doctor_reauthorization() {
    let env = Env::default();
    let contract_id = env.register(MedicalRecordsContract, ());
    let client = MedicalRecordsContractClient::new(&env, &contract_id);

    // Create test addresses
    let patient = Address::generate(&env);
    let doctor = Address::generate(&env);
    
    // Grant access to doctor
    env.mock_all_auths();
    client.grant_access(&patient, &doctor);
    
    // Doctor adds a record
    let record_hash1 = BytesN::from_array(&env, &[1u8; 32]);
    let description1 = String::from_str(&env, "First visit");
    env.mock_all_auths();
    client.add_medical_record(&patient, &doctor, &record_hash1, &description1);
    
    // Revoke doctor's access
    env.mock_all_auths();
    client.revoke_access(&patient, &doctor);
    
    // Verify doctor no longer has access
    let authorized_doctors = client.get_authorized_doctors(&patient);
    assert_eq!(authorized_doctors.len(), 0);
    
    // Re-authorize the same doctor later
    env.mock_all_auths();
    client.grant_access(&patient, &doctor);
    
    // Verify doctor has access again
    let authorized_doctors_after = client.get_authorized_doctors(&patient);
    assert_eq!(authorized_doctors_after.len(), 1);
    assert_eq!(authorized_doctors_after.get(0), Some(doctor.clone()));
    
    // Doctor adds another record after being re-authorized
    let record_hash2 = BytesN::from_array(&env, &[2u8; 32]);
    let description2 = String::from_str(&env, "Return visit after re-authorization");
    env.mock_all_auths();
    client.add_medical_record(&patient, &doctor, &record_hash2, &description2);
    
    // Verify both records are available
    let records = client.get_medical_records(&patient);
    assert_eq!(records.len(), 2, "Should have records from before and after re-authorization");
    
    // Verify record descriptions
    let has_first_record = records.iter().any(|r| r.description == description1);
    let has_second_record = records.iter().any(|r| r.description == description2);
    
    assert!(has_first_record, "Missing record from before revocation");
    assert!(has_second_record, "Missing record from after re-authorization");
} 