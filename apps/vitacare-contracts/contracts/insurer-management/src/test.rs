#![cfg(test)]

use crate::{InsurerManagement, InsurerManagementClient};
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    Address, Env, IntoVal, String,
};

/// Helper function to create a test environment and contract client
fn setup_test() -> (Env, Address, InsurerManagementClient<'static>) {
    let env = Env::default();
    // Set up a mock ledger with a non-zero timestamp
    env.ledger().set_timestamp(12345);
    let contract_id = env.register(InsurerManagement, ());
    let client = InsurerManagementClient::new(&env, &contract_id);
    (env, contract_id, client)
}

/// Helper function to register a test insurer
fn register_test_insurer(
    client: &InsurerManagementClient,
    env: &Env,
    insurer_address: &Address,
) -> (String, String, String) {
    let name = "Blue Cross Insurance";
    let license_id = "INS123456";
    let metadata = r#"{"email":"contact@bluecross.example","phone":"+1234567890","website":"https://bluecross.example","address":"123 Health St, Medical City"}"#;

    client.register_insurer(
        insurer_address,
        &name.into_val(env),
        &license_id.into_val(env),
        &metadata.into_val(env),
    );

    (
        String::from_str(env, name),
        String::from_str(env, license_id),
        String::from_str(env, metadata),
    )
}

#[test]
fn test_register_and_get_insurer() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create test data
    let insurer_address = Address::generate(&env);
    let (name, license_id, metadata) = register_test_insurer(&client, &env, &insurer_address);

    // Get the insurer data
    let result = client.get_insurer(&insurer_address);

    // Verify the data is correct
    assert_eq!(result.wallet, insurer_address);
    assert_eq!(result.name, name.into_val(&env));
    assert_eq!(result.license_id, license_id.into_val(&env));
    assert_eq!(result.metadata, metadata.into_val(&env));
    assert_eq!(result.coverage_policies.len(), 0);
    assert_eq!(result.claims_reviewers.len(), 0);
    assert!(result.active);
    assert!(result.registered_at > 0);
    assert!(result.updated_at > 0);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_register_insurer_already_exists() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create test data
    let insurer_address = Address::generate(&env);

    // Register the insurer the first time (should succeed)
    register_test_insurer(&client, &env, &insurer_address);

    // Try to register the same insurer again (should panic with InsurerAlreadyExists error)
    register_test_insurer(&client, &env, &insurer_address);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_register_insurer_invalid_inputs() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create test data with empty name (invalid)
    let insurer_address = Address::generate(&env);
    let empty_name = "";
    let license_id = "INS123456";
    let metadata = r#"{"email":"contact@bluecross.example"}"#;

    // Try to register the insurer with invalid inputs (should panic with InvalidInput error)
    client.register_insurer(
        &insurer_address,
        &empty_name.into_val(&env),
        &license_id.into_val(&env),
        &metadata.into_val(&env),
    );
}

#[test]
fn test_update_insurer() {
    // Set up the test environment
    let (env, _, client) = setup_test();

    // Create and register test insurer
    let insurer_address = Address::generate(&env);
    register_test_insurer(&client, &env, &insurer_address);

    // Update the metadata
    let new_metadata = r#"{"email":"support@bluecross.example","phone":"+1987654321","website":"https://bluecross.example","address":"456 Wellness Blvd, Medical City"}"#;

    // Set up authorization for the call
    env.mock_all_auths();

    // Update the insurer's metadata
    client.update_insurer(&insurer_address, &new_metadata.into_val(&env));

    // Get the updated insurer data
    let result = client.get_insurer(&insurer_address);

    // Verify the update was successful
    assert_eq!(result.metadata, new_metadata.into_val(&env));
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_update_insurer_not_found() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create a non-registered insurer address
    let insurer_address = Address::generate(&env);

    // Set up authorization for the call
    env.mock_all_auths();

    // Try to update a non-existent insurer (should panic with InsurerNotFound error)
    let new_metadata = r#"{"email":"support@bluecross.example"}"#;
    client.update_insurer(&insurer_address, &new_metadata.into_val(&env));
}

#[test]
#[should_panic(expected = "Error(Auth, InvalidAction)")]
fn test_update_insurer_unauthorized() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create and register test insurer
    let insurer_address = Address::generate(&env);
    register_test_insurer(&client, &env, &insurer_address);

    // Try to update the insurer without authorization (should panic with auth error)
    let new_metadata = r#"{"email":"support@bluecross.example"}"#;
    client.update_insurer(&insurer_address, &new_metadata.into_val(&env));
}

#[test]
fn test_add_coverage_policy() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create and register test insurer
    let insurer_address = Address::generate(&env);
    register_test_insurer(&client, &env, &insurer_address);

    // Set up authorization for the call
    env.mock_all_auths();

    // Add a coverage policy
    let policy_id = "POL001";
    let policy_name = "Basic Health Plan";
    let policy_description = "Covers basic medical expenses";
    let premium_amount = 50000; // $500.00
    let coverage_amount = 10000000; // $100,000.00
    let duration_days = 365; // 1 year

    client.add_coverage_policy(
        &insurer_address,
        &policy_id.into_val(&env),
        &policy_name.into_val(&env),
        &policy_description.into_val(&env),
        &premium_amount,
        &coverage_amount,
        &duration_days,
    );

    // Get the insurer data with the policy
    let result = client.get_insurer(&insurer_address);

    // Verify the policy was added
    assert_eq!(result.coverage_policies.len(), 1);
    let policy = result.coverage_policies.get(0).unwrap();
    assert_eq!(policy.id, policy_id.into_val(&env));
    assert_eq!(policy.name, policy_name.into_val(&env));
    assert_eq!(policy.description, policy_description.into_val(&env));
    assert_eq!(policy.premium_amount, premium_amount);
    assert_eq!(policy.coverage_amount, coverage_amount);
    assert_eq!(policy.duration_days, duration_days);
}

#[test]
fn test_add_duplicate_policy() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create and register test insurer
    let insurer_address = Address::generate(&env);
    register_test_insurer(&client, &env, &insurer_address);

    // Set up authorization for the calls
    env.mock_all_auths();

    // Policy details
    let policy_id = "POL001";
    let policy_name = "Basic Health Plan";
    let policy_description = "Covers basic medical expenses";
    let premium_amount = 50000;
    let coverage_amount = 10000000;
    let duration_days = 365;

    // Add a policy
    client.add_coverage_policy(
        &insurer_address,
        &policy_id.into_val(&env),
        &policy_name.into_val(&env),
        &policy_description.into_val(&env),
        &premium_amount,
        &coverage_amount,
        &duration_days,
    );

    // Try to add the same policy again (should succeed but not add a duplicate)
    client.add_coverage_policy(
        &insurer_address,
        &policy_id.into_val(&env),
        &policy_name.into_val(&env),
        &policy_description.into_val(&env),
        &premium_amount,
        &coverage_amount,
        &duration_days,
    );

    // Get the insurer data
    let result = client.get_insurer(&insurer_address);

    // Verify only one policy was added
    assert_eq!(result.coverage_policies.len(), 1);
}

#[test]
fn test_update_coverage_policy() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create and register test insurer
    let insurer_address = Address::generate(&env);
    register_test_insurer(&client, &env, &insurer_address);

    // Set up authorization for the calls
    env.mock_all_auths();

    // Add a policy
    let policy_id = "POL001";
    let policy_name = "Basic Health Plan";
    let policy_description = "Covers basic medical expenses";
    let premium_amount = 50000;
    let coverage_amount = 10000000;
    let duration_days = 365;

    client.add_coverage_policy(
        &insurer_address,
        &policy_id.into_val(&env),
        &policy_name.into_val(&env),
        &policy_description.into_val(&env),
        &premium_amount,
        &coverage_amount,
        &duration_days,
    );

    // Update the policy
    let new_policy_name = "Premium Health Plan";
    let new_policy_description = "Covers advanced medical expenses";
    let new_premium_amount = 100000; // $1,000.00
    let new_coverage_amount = 20000000; // $200,000.00
    let new_duration_days = 730; // 2 years

    client.update_coverage_policy(
        &insurer_address,
        &policy_id.into_val(&env),
        &new_policy_name.into_val(&env),
        &new_policy_description.into_val(&env),
        &new_premium_amount,
        &new_coverage_amount,
        &new_duration_days,
    );

    // Get the insurer data
    let result = client.get_insurer(&insurer_address);

    // Verify the policy was updated
    assert_eq!(result.coverage_policies.len(), 1);
    let updated_policy = result.coverage_policies.get(0).unwrap();
    assert_eq!(updated_policy.id, policy_id.into_val(&env));
    assert_eq!(updated_policy.name, new_policy_name.into_val(&env));
    assert_eq!(
        updated_policy.description,
        new_policy_description.into_val(&env)
    );
    assert_eq!(updated_policy.premium_amount, new_premium_amount);
    assert_eq!(updated_policy.coverage_amount, new_coverage_amount);
    assert_eq!(updated_policy.duration_days, new_duration_days);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_update_nonexistent_policy() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create and register test insurer
    let insurer_address = Address::generate(&env);
    register_test_insurer(&client, &env, &insurer_address);

    // Set up authorization for the call
    env.mock_all_auths();

    // Try to update a non-existent policy (should panic with error)
    let nonexistent_policy_id = "NONEXISTENT";
    let policy_name = "Premium Health Plan";

    client.update_coverage_policy(
        &insurer_address,
        &nonexistent_policy_id.into_val(&env),
        &policy_name.into_val(&env),
        &"Description".into_val(&env),
        &100000,
        &20000000,
        &730,
    );
}

#[test]
fn test_add_claims_reviewer() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create and register test insurer
    let insurer_address = Address::generate(&env);
    register_test_insurer(&client, &env, &insurer_address);

    // Set up authorization for the call
    env.mock_all_auths();

    // Add a claims reviewer
    let reviewer_address = Address::generate(&env);
    let reviewer_name = "Dr. Jane Smith";
    let reviewer_role = "Medical Specialist";

    client.add_claims_reviewer(
        &insurer_address,
        &reviewer_address,
        &reviewer_name.into_val(&env),
        &reviewer_role.into_val(&env),
    );

    // Get the insurer data with the reviewer
    let result = client.get_insurer(&insurer_address);

    // Verify the reviewer was added
    assert_eq!(result.claims_reviewers.len(), 1);
    let reviewer = result.claims_reviewers.get(0).unwrap();
    assert_eq!(reviewer.wallet, reviewer_address);
    assert_eq!(reviewer.name, reviewer_name.into_val(&env));
    assert_eq!(reviewer.role, reviewer_role.into_val(&env));
    assert!(reviewer.active);
}

#[test]
fn test_add_duplicate_reviewer() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create and register test insurer
    let insurer_address = Address::generate(&env);
    register_test_insurer(&client, &env, &insurer_address);

    // Set up authorization for the calls
    env.mock_all_auths();

    // Add a claims reviewer
    let reviewer_address = Address::generate(&env);
    let reviewer_name = "Dr. Jane Smith";
    let reviewer_role = "Medical Specialist";

    client.add_claims_reviewer(
        &insurer_address,
        &reviewer_address,
        &reviewer_name.into_val(&env),
        &reviewer_role.into_val(&env),
    );

    // Try to add the same reviewer again (should succeed but not add a duplicate)
    client.add_claims_reviewer(
        &insurer_address,
        &reviewer_address,
        &reviewer_name.into_val(&env),
        &reviewer_role.into_val(&env),
    );

    // Get the insurer data
    let result = client.get_insurer(&insurer_address);

    // Verify only one reviewer was added
    assert_eq!(result.claims_reviewers.len(), 1);
}

#[test]
fn test_update_claims_reviewer() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create and register test insurer
    let insurer_address = Address::generate(&env);
    register_test_insurer(&client, &env, &insurer_address);

    // Set up authorization for the calls
    env.mock_all_auths();

    // Add a claims reviewer
    let reviewer_address = Address::generate(&env);
    let reviewer_name = "Dr. Jane Smith";
    let reviewer_role = "Medical Specialist";

    client.add_claims_reviewer(
        &insurer_address,
        &reviewer_address,
        &reviewer_name.into_val(&env),
        &reviewer_role.into_val(&env),
    );

    // Update the reviewer
    let new_reviewer_name = "Dr. Jane Smith-Johnson";
    let new_reviewer_role = "Chief Medical Officer";
    let active = false; // Deactivate the reviewer

    client.update_claims_reviewer(
        &insurer_address,
        &reviewer_address,
        &new_reviewer_name.into_val(&env),
        &new_reviewer_role.into_val(&env),
        &active,
    );

    // Get the insurer data
    let result = client.get_insurer(&insurer_address);

    // Verify the reviewer was updated
    assert_eq!(result.claims_reviewers.len(), 1);
    let updated_reviewer = result.claims_reviewers.get(0).unwrap();
    assert_eq!(updated_reviewer.wallet, reviewer_address);
    assert_eq!(updated_reviewer.name, new_reviewer_name.into_val(&env));
    assert_eq!(updated_reviewer.role, new_reviewer_role.into_val(&env));
    assert_eq!(updated_reviewer.active, active);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_update_nonexistent_reviewer() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create and register test insurer
    let insurer_address = Address::generate(&env);
    register_test_insurer(&client, &env, &insurer_address);

    // Set up authorization for the call
    env.mock_all_auths();

    // Try to update a non-existent reviewer (should panic with error)
    let nonexistent_reviewer_address = Address::generate(&env);

    client.update_claims_reviewer(
        &insurer_address,
        &nonexistent_reviewer_address,
        &"Dr. Nobody".into_val(&env),
        &"Ghost Role".into_val(&env),
        &true,
    );
}

#[test]
fn test_deactivate_and_reactivate_insurer() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create and register test insurer
    let insurer_address = Address::generate(&env);
    register_test_insurer(&client, &env, &insurer_address);

    // Set up authorization for the calls
    env.mock_all_auths();

    // Deactivate the insurer
    client.deactivate_insurer(&insurer_address);

    // Verify the insurer is deactivated
    let result = client.get_insurer(&insurer_address);
    assert!(!result.active);

    // Reactivate the insurer
    client.reactivate_insurer(&insurer_address);

    // Verify the insurer is reactivated
    let result = client.get_insurer(&insurer_address);
    assert!(result.active);
}

#[test]
fn test_get_all_insurers() {
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Initially, there should be no insurers
    let initial_insurers = client.get_all_insurers();
    assert_eq!(initial_insurers.len(), 0);

    // Register three insurers
    let insurer1 = Address::generate(&env);
    let insurer2 = Address::generate(&env);
    let insurer3 = Address::generate(&env);

    register_test_insurer(&client, &env, &insurer1);
    register_test_insurer(&client, &env, &insurer2);
    register_test_insurer(&client, &env, &insurer3);

    // Get all insurers
    let all_insurers = client.get_all_insurers();

    // Verify all three insurers were registered
    assert_eq!(all_insurers.len(), 3);
    assert!(all_insurers.contains(&insurer1));
    assert!(all_insurers.contains(&insurer2));
    assert!(all_insurers.contains(&insurer3));
}

#[test]
fn test_data_key_functionality() {
    // This helps verify that the DataKey enum works correctly with storage.
    // Set up the test environment
    let (env, _contract_id, client) = setup_test();

    // Create and register multiple insurers
    let insurer1 = Address::generate(&env);
    let insurer2 = Address::generate(&env);

    register_test_insurer(&client, &env, &insurer1);
    register_test_insurer(&client, &env, &insurer2);

    // Get both insurers
    let data1 = client.get_insurer(&insurer1);
    let data2 = client.get_insurer(&insurer2);

    // Verify the DataKey is correctly mapping different wallets to different storage keys
    assert_ne!(data1.wallet, data2.wallet);

    // Register the same insurers again to verify IdMap works correctly
    let all_insurers = client.get_all_insurers();
    assert_eq!(all_insurers.len(), 2); // Still just 2 insurers since we haven't added any new ones
}
