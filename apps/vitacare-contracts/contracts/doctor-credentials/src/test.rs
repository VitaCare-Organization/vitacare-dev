#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, vec, Env};

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