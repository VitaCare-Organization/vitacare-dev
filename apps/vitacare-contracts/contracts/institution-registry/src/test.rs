#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_institution_registration() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InstitutionRegistry);
    let client = InstitutionRegistryClient::new(&env, &contract_id);
    
    let institution = Address::generate(&env);
    let name = String::from_str(&env, "General Hospital");
    let license_id = String::from_str(&env, "GH12345");
    let metadata = String::from_str(&env, "{\"address\":\"123 Main St\",\"phone\":\"555-1234\",\"specialties\":[\"surgery\",\"cardiology\"]}");
    
    // Register the institution
    env.mock_all_auths();
    let result = client.register_institution(&institution, &name, &license_id, &metadata);
    
    assert_eq!(result.name, name);
    assert_eq!(result.license_id, license_id);
    assert_eq!(result.metadata, metadata);
    assert_eq!(result.verified, false);
    
    // Get the institution data
    let data = client.get_institution(&institution);
    assert_eq!(data.name, name);
    assert_eq!(data.license_id, license_id);
    assert_eq!(data.metadata, metadata);
    assert_eq!(data.verified, false);
}

#[test]
fn test_registration_requires_auth() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InstitutionRegistry);
    let client = InstitutionRegistryClient::new(&env, &contract_id);
    
    let institution = Address::generate(&env);
    let name = String::from_str(&env, "General Hospital");
    let license_id = String::from_str(&env, "GH12345");
    let metadata = String::from_str(&env, "{\"address\":\"123 Main St\",\"phone\":\"555-1234\"}");
    
    // Test failure without authentication
    let result = client.try_register_institution(&institution, &name, &license_id, &metadata);
    assert!(result.is_err());
    
    // Test success with authentication
    env.mock_all_auths();
    let result = client.register_institution(&institution, &name, &license_id, &metadata);
    assert_eq!(result.name, name);
}

#[test]
fn test_duplicate_registration() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InstitutionRegistry);
    let client = InstitutionRegistryClient::new(&env, &contract_id);
    
    let institution = Address::generate(&env);
    let name = String::from_str(&env, "General Hospital");
    let license_id = String::from_str(&env, "GH12345");
    let metadata = String::from_str(&env, "{\"address\":\"123 Main St\",\"phone\":\"555-1234\"}");
    
    // Register once
    env.mock_all_auths();
    client.register_institution(&institution, &name, &license_id, &metadata);
    
    // Try to register again - should error with InstitutionAlreadyRegistered
    let result = client.try_register_institution(&institution, &name, &license_id, &metadata);
    assert!(result.is_err());
}

#[test]
fn test_institution_update() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InstitutionRegistry);
    let client = InstitutionRegistryClient::new(&env, &contract_id);
    
    let institution = Address::generate(&env);
    let name = String::from_str(&env, "General Hospital");
    let license_id = String::from_str(&env, "GH12345");
    let metadata = String::from_str(&env, "{\"address\":\"123 Main St\",\"phone\":\"555-1234\"}");
    
    // Register the institution
    env.mock_all_auths();
    client.register_institution(&institution, &name, &license_id, &metadata);
    
    // Update the institution metadata
    let new_metadata = String::from_str(&env, "{\"address\":\"123 Main St\",\"phone\":\"555-5678\",\"website\":\"www.generalhospital.com\"}");
    let result = client.update_institution(&institution, &new_metadata);
    
    assert_eq!(result.metadata, new_metadata);
    
    // Get the institution data to verify update
    let data = client.get_institution(&institution);
    assert_eq!(data.metadata, new_metadata);
}

#[test]
fn test_update_requires_auth() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InstitutionRegistry);
    let client = InstitutionRegistryClient::new(&env, &contract_id);
    
    let institution = Address::generate(&env);
    let name = String::from_str(&env, "General Hospital");
    let license_id = String::from_str(&env, "GH12345");
    let metadata = String::from_str(&env, "{\"address\":\"123 Main St\",\"phone\":\"555-1234\"}");
    
    // Register the institution
    env.mock_all_auths();
    client.register_institution(&institution, &name, &license_id, &metadata);
    
    // Clear the auth to test without auth
    env.mock_auths(&[]);
    
    // Try to update without auth
    let new_metadata = String::from_str(&env, "{\"address\":\"123 Main St\",\"phone\":\"555-5678\"}");
    let result = client.try_update_institution(&institution, &new_metadata);
    assert!(result.is_err());
    
    // Update with auth
    env.mock_all_auths();
    let new_metadata = String::from_str(&env, "{\"address\":\"123 Main St\",\"phone\":\"555-5678\"}");
    let result = client.update_institution(&institution, &new_metadata);
    assert_eq!(result.metadata, new_metadata);
}

#[test]
fn test_update_nonexistent_institution() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InstitutionRegistry);
    let client = InstitutionRegistryClient::new(&env, &contract_id);
    
    let institution = Address::generate(&env);
    let metadata = String::from_str(&env, "{\"address\":\"123 Main St\",\"phone\":\"555-1234\"}");
    
    // Try to update a non-existent institution - should fail with InstitutionNotFound
    env.mock_all_auths();
    let result = client.try_update_institution(&institution, &metadata);
    assert!(result.is_err());
}

#[test]
fn test_institution_verification() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InstitutionRegistry);
    let client = InstitutionRegistryClient::new(&env, &contract_id);
    
    // Set admin
    let admin = Address::generate(&env);
    env.mock_all_auths();
    client.set_admin(&admin);
    
    // Register an institution
    let institution = Address::generate(&env);
    let name = String::from_str(&env, "General Hospital");
    let license_id = String::from_str(&env, "GH12345");
    let metadata = String::from_str(&env, "{\"address\":\"123 Main St\",\"phone\":\"555-1234\"}");
    
    client.register_institution(&institution, &name, &license_id, &metadata);
    
    // Verify the institution
    let result = client.verify_institution(&admin, &institution);
    
    assert_eq!(result.verified, true);
    
    // Get the institution data to verify the verification
    let data = client.get_institution(&institution);
    assert_eq!(data.verified, true);
}

#[test]
fn test_verification_requires_admin_auth() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InstitutionRegistry);
    let client = InstitutionRegistryClient::new(&env, &contract_id);
    
    // Set admin
    let admin = Address::generate(&env);
    env.mock_all_auths();
    client.set_admin(&admin);
    
    // Register an institution
    let institution = Address::generate(&env);
    let name = String::from_str(&env, "General Hospital");
    let license_id = String::from_str(&env, "GH12345");
    let metadata = String::from_str(&env, "{\"address\":\"123 Main St\",\"phone\":\"555-1234\"}");
    
    client.register_institution(&institution, &name, &license_id, &metadata);
    
    // Clear the auth to test without auth
    env.mock_auths(&[]);
    
    // Try to verify without authentication
    let result = client.try_verify_institution(&admin, &institution);
    assert!(result.is_err());
    
    // Verify with authentication
    env.mock_all_auths();
    let result = client.verify_institution(&admin, &institution);
    assert_eq!(result.verified, true);
}

#[test]
fn test_verification_by_non_admin() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InstitutionRegistry);
    let client = InstitutionRegistryClient::new(&env, &contract_id);
    
    // Set admin
    let admin = Address::generate(&env);
    env.mock_all_auths();
    client.set_admin(&admin);
    
    // Register an institution
    let institution = Address::generate(&env);
    let name = String::from_str(&env, "General Hospital");
    let license_id = String::from_str(&env, "GH12345");
    let metadata = String::from_str(&env, "{\"address\":\"123 Main St\",\"phone\":\"555-1234\"}");
    
    client.register_institution(&institution, &name, &license_id, &metadata);
    
    // Try to verify with a non-admin
    let fake_admin = Address::generate(&env);
    env.mock_all_auths();
    let result = client.try_verify_institution(&fake_admin, &institution);
    assert!(result.is_err());
}

#[test]
fn test_get_nonexistent_institution() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InstitutionRegistry);
    let client = InstitutionRegistryClient::new(&env, &contract_id);
    
    let non_existent = Address::generate(&env);
    
    // Try to get a non-existent institution
    let result = client.try_get_institution(&non_existent);
    assert!(result.is_err());
}

#[test]
fn test_set_and_change_admin() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InstitutionRegistry);
    let client = InstitutionRegistryClient::new(&env, &contract_id);
    
    // Set initial admin
    let admin1 = Address::generate(&env);
    env.mock_all_auths();
    client.set_admin(&admin1);
    
    // Register an institution
    let institution = Address::generate(&env);
    let name = String::from_str(&env, "Test Clinic");
    let license_id = String::from_str(&env, "TC6789");
    let metadata = String::from_str(&env, "{}");
    
    client.register_institution(&institution, &name, &license_id, &metadata);
    
    // Verify the institution with admin1
    env.mock_all_auths();
    let result = client.verify_institution(&admin1, &institution);
    assert_eq!(result.verified, true);
    
    // Change admin to admin2
    let admin2 = Address::generate(&env);
    env.mock_all_auths();
    client.set_admin(&admin2);
    
    // Register another institution
    let institution2 = Address::generate(&env);
    env.mock_all_auths();
    client.register_institution(&institution2, &name, &license_id, &metadata);
    
    // Verify institution2 with admin2
    env.mock_all_auths();
    let result = client.verify_institution(&admin2, &institution2);
    assert_eq!(result.verified, true);
    
    // Verify admin1 can no longer verify
    env.mock_all_auths();
    let result = client.try_verify_institution(&admin1, &institution2);
    assert!(result.is_err());
}

#[test]
fn test_change_admin_requires_current_admin_auth() {
    let env = Env::default();
    let contract_id = env.register_contract(None, InstitutionRegistry);
    let client = InstitutionRegistryClient::new(&env, &contract_id);
    
    // Set initial admin
    let admin1 = Address::generate(&env);
    env.mock_all_auths();
    client.set_admin(&admin1);
    
    // Try to change admin without auth
    let admin2 = Address::generate(&env);
    env.mock_auths(&[]);
    let result = client.try_set_admin(&admin2);
    assert!(result.is_err());
    
    // Change admin with auth
    env.mock_all_auths();
    client.set_admin(&admin2);
    
    // Verify admin2 is now the admin
    let institution = Address::generate(&env);
    let name = String::from_str(&env, "Test Clinic");
    let license_id = String::from_str(&env, "TC6789");
    let metadata = String::from_str(&env, "{}");
    
    env.mock_all_auths();
    client.register_institution(&institution, &name, &license_id, &metadata);
    
    // Verify institution with admin2
    env.mock_all_auths();
    let result = client.verify_institution(&admin2, &institution);
    assert_eq!(result.verified, true);
} 