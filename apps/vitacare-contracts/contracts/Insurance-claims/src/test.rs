#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, vec, Env};

#[test]
fn test_insurance_claims_flow() {
    let env = Env::default();
    
    // Create test addresses
    let admin = Address::generate(&env);
    let insurer = Address::generate(&env);
    let patient = Address::generate(&env);
    
    // Register contract
    let contract_id = env.register(InsuranceClaims, ());
    let client = InsuranceClaimsClient::new(&env, &contract_id);
    
    // Initialize contract with admin and insurers
    env.mock_all_auths();
    let insurers = vec![&env, insurer.clone()];
    client.initialize(&admin, &insurers);
    
    // Patient submits a claim
    let service_id = 12345u64;
    let cost = 500_i128;
    
    // Submit claim
    env.mock_all_auths();
    let claim_id = client.submit_claim(&patient, &service_id, &cost);
    assert_eq!(claim_id, 0);
    
    // Check claim status is pending
    let status = client.get_claim_status(&claim_id);
    assert_eq!(status, ClaimStatus::Pending);
    
    // Get full claim details
    let claim = client.get_claim_details(&claim_id);
    assert_eq!(claim.patient, patient);
    assert_eq!(claim.service_id, service_id);
    assert_eq!(claim.cost, cost);
    assert_eq!(claim.status, ClaimStatus::Pending);
    assert_eq!(claim.insurer, None);
    
    // Process claim (approve)
    env.mock_all_auths();
    client.process_claim(&insurer, &claim_id, &true);
    
    // Check status updated to approved
    let status = client.get_claim_status(&claim_id);
    assert_eq!(status, ClaimStatus::Approved);
    
    // Get updated claim details
    let claim = client.get_claim_details(&claim_id);
    assert_eq!(claim.status, ClaimStatus::Approved);
    assert_eq!(claim.insurer, Some(insurer));
}

#[test]
fn test_patient_claims_tracking() {
    let env = Env::default();
    
    // Create test addresses
    let admin = Address::generate(&env);
    let insurer = Address::generate(&env);
    let patient = Address::generate(&env);
    
    // Register contract
    let contract_id = env.register(InsuranceClaims, ());
    let client = InsuranceClaimsClient::new(&env, &contract_id);
    
    // Initialize contract
    env.mock_all_auths();
    let insurers = vec![&env, insurer.clone()];
    client.initialize(&admin, &insurers);
    
    // Submit multiple claims
    env.mock_all_auths();
    let claim_id1 = client.submit_claim(&patient, &1001u64, &100_i128);
    env.mock_all_auths();
    let claim_id2 = client.submit_claim(&patient, &1002u64, &200_i128);
    env.mock_all_auths();
    let claim_id3 = client.submit_claim(&patient, &1003u64, &300_i128);
    
    // Get patient claims
    let patient_claims = client.get_patient_claims(&patient);
    assert_eq!(patient_claims.len(), 3);
    assert!(patient_claims.contains(&claim_id1));
    assert!(patient_claims.contains(&claim_id2));
    assert!(patient_claims.contains(&claim_id3));
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_invalid_amount() {
    let env = Env::default();
    
    // Create test addresses
    let admin = Address::generate(&env);
    let insurer = Address::generate(&env);
    let patient = Address::generate(&env);
    
    // Register contract
    let contract_id = env.register(InsuranceClaims, ());
    let client = InsuranceClaimsClient::new(&env, &contract_id);
    
    // Initialize contract
    env.mock_all_auths();
    let insurers = vec![&env, insurer.clone()];
    client.initialize(&admin, &insurers);
    
    // Invalid amount should fail
    env.mock_all_auths();
    client.submit_claim(&patient, &1001u64, &0_i128);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_unauthorized_insurer() {
    let env = Env::default();
    
    // Create test addresses
    let admin = Address::generate(&env);
    let insurer = Address::generate(&env);
    let unauthorized = Address::generate(&env);
    let patient = Address::generate(&env);
    
    // Register contract
    let contract_id = env.register(InsuranceClaims, ());
    let client = InsuranceClaimsClient::new(&env, &contract_id);
    
    // Initialize contract
    env.mock_all_auths();
    let insurers = vec![&env, insurer.clone()];
    client.initialize(&admin, &insurers);
    
    // Submit a valid claim
    env.mock_all_auths();
    let claim_id = client.submit_claim(&patient, &1001u64, &100_i128);
    
    // Unauthorized insurer should fail
    env.mock_all_auths();
    client.process_claim(&unauthorized, &claim_id, &true);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_already_processed() {
    let env = Env::default();
    
    // Create test addresses
    let admin = Address::generate(&env);
    let insurer = Address::generate(&env);
    let patient = Address::generate(&env);
    
    // Register contract
    let contract_id = env.register(InsuranceClaims, ());
    let client = InsuranceClaimsClient::new(&env, &contract_id);
    
    // Initialize contract
    env.mock_all_auths();
    let insurers = vec![&env, insurer.clone()];
    client.initialize(&admin, &insurers);
    
    // Submit a valid claim
    env.mock_all_auths();
    let claim_id = client.submit_claim(&patient, &1001u64, &100_i128);
    
    // Process claim once
    env.mock_all_auths();
    client.process_claim(&insurer, &claim_id, &true);
    
    // Trying to process the same claim again should fail
    env.mock_all_auths();
    client.process_claim(&insurer, &claim_id, &false);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_claim_not_found() {
    let env = Env::default();
    
    // Create test addresses
    let admin = Address::generate(&env);
    let insurer = Address::generate(&env);
    
    // Register contract
    let contract_id = env.register(InsuranceClaims, ());
    let client = InsuranceClaimsClient::new(&env, &contract_id);
    
    // Initialize contract
    env.mock_all_auths();
    let insurers = vec![&env, insurer.clone()];
    client.initialize(&admin, &insurers);
    
    // Non-existent claim should fail
    client.get_claim_status(&999u64);
}

#[test]
fn test_admin_functions() {
    let env = Env::default();
    
    // Create test addresses
    let admin = Address::generate(&env);
    let insurer1 = Address::generate(&env);
    let insurer2 = Address::generate(&env);
    
    // Register contract
    let contract_id = env.register(InsuranceClaims, ());
    let client = InsuranceClaimsClient::new(&env, &contract_id);
    
    // Initialize contract with one insurer
    env.mock_all_auths();
    let insurers = vec![&env, insurer1.clone()];
    client.initialize(&admin, &insurers);
    
    // Admin can add a new insurer
    env.mock_all_auths();
    client.add_insurer(&admin, &insurer2);
    
    // Test new insurer can process claims
    env.mock_all_auths();
    let patient = Address::generate(&env);
    let claim_id = client.submit_claim(&patient, &1001u64, &100_i128);
    
    // New insurer should be authorized
    env.mock_all_auths();
    client.process_claim(&insurer2, &claim_id, &true);
}