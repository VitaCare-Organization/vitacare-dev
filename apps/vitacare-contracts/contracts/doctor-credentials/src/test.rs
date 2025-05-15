#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, vec, Env};
use types::ContractError;

#[test]
fn test_doctor_registration() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DoctorCredentials);
    let client = DoctorCredentialsClient::new(&env, &contract_id);

    let doctor = Address::generate(&env);
    let name = String::from_str(&env, "Dr. John Doe");
    let specialization = String::from_str(&env, "Cardiology");
    let certificate_hash = Bytes::from_slice(&env, &[1, 2, 3, 4]);

    // Test successful registration
    client.register_doctor(&doctor, &name, &specialization, &certificate_hash);
    
    // Test getting doctor data
    let doctor_data = client.get_doctor(&doctor);
    assert_eq!(doctor_data.name, name);
    assert_eq!(doctor_data.specialization, specialization);
    assert_eq!(doctor_data.certificate_hash, certificate_hash);
    assert_eq!(doctor_data.is_verified, false);
    assert_eq!(doctor_data.verified_by, None);

    // Test duplicate registration
    let result = client.try_register_doctor(&doctor, &name, &specialization, &certificate_hash);
    assert!(result.is_err());
}

#[test]
fn test_doctor_verification() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DoctorCredentials);
    let client = DoctorCredentialsClient::new(&env, &contract_id);

    let doctor = Address::generate(&env);
    let institution = Address::generate(&env);
    let name = String::from_str(&env, "Dr. John Doe");
    let specialization = String::from_str(&env, "Cardiology");
    let certificate_hash = Bytes::from_slice(&env, &[1, 2, 3, 4]);

    // Register doctor
    client.register_doctor(&doctor, &name, &specialization, &certificate_hash);

    // Test verification with unverified institution
    let result = client.try_verify_doctor(&doctor, &institution);
    assert!(result.is_err());

    // Add verified institution
    client.add_institution(&institution);

    // Test successful verification
    client.verify_doctor(&doctor, &institution);

    // Verify doctor data
    let doctor_data = client.get_doctor(&doctor);
    assert_eq!(doctor_data.is_verified, true);
    assert_eq!(doctor_data.verified_by, Some(institution));
}

#[test]
fn test_invalid_inputs() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DoctorCredentials);
    let client = DoctorCredentialsClient::new(&env, &contract_id);

    let doctor = Address::generate(&env);
    let empty_name = String::from_str(&env, "");
    let empty_specialization = String::from_str(&env, "");
    let empty_hash = Bytes::from_slice(&env, &[]);
    let valid_name = String::from_str(&env, "Dr. John Doe");
    let valid_specialization = String::from_str(&env, "Cardiology");
    let valid_hash = Bytes::from_slice(&env, &[1, 2, 3, 4]);

    // Test empty name
    let result = client.try_register_doctor(&doctor, &empty_name, &valid_specialization, &valid_hash);
    assert!(result.is_err());

    // Test empty specialization
    let result = client.try_register_doctor(&doctor, &valid_name, &empty_specialization, &valid_hash);
    assert!(result.is_err());

    // Test empty certificate hash
    let result = client.try_register_doctor(&doctor, &valid_name, &valid_specialization, &empty_hash);
    assert!(result.is_err());
}

#[test]
fn test_nonexistent_doctor() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DoctorCredentials);
    let client = DoctorCredentialsClient::new(&env, &contract_id);

    let doctor = Address::generate(&env);
    let institution = Address::generate(&env);

    // Test getting nonexistent doctor
    let result = client.try_get_doctor(&doctor);
    assert!(result.is_err());

    // Test verifying nonexistent doctor
    let result = client.try_verify_doctor(&doctor, &institution);
    assert!(result.is_err());
} 