#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String, Vec,
};

use crate::{
    DoctorManagementContract, DoctorManagementContractClient,
};

fn create_test_contract(e: &Env) -> DoctorManagementContractClient {
    // Set up initial ledger state
    e.ledger().with_mut(|l| {
        l.timestamp = 1234567890;
    });
    
    let contract_id = e.register(DoctorManagementContract {}, ());
    DoctorManagementContractClient::new(e, &contract_id)
}

fn create_test_doctor(
    e: &Env,
    contract: &DoctorManagementContractClient,
    wallet: &Address,
    name: &str,
    specialization: &str,
    institution: &Address,
) {
    contract.create_doctor_profile(
        wallet,
        &String::from_str(e, name),
        &String::from_str(e, specialization),
        institution,
    );
}

#[test]
fn test_create_doctor_profile() {
    let e = Env::default();
    let contract = create_test_contract(&e);
    
    let wallet = Address::generate(&e);
    let institution = Address::generate(&e);
    let name = "Dr. John Doe";
    let specialization = "Cardiology";

    // Test successful creation
    contract.create_doctor_profile(
        &wallet,
        &String::from_str(&e, name),
        &String::from_str(&e, specialization),
        &institution,
    );

    // Verify the profile was created correctly
    let profile = contract.get_doctor_profile(&wallet);
    assert_eq!(profile.wallet, wallet);
    assert_eq!(profile.name, String::from_str(&e, name));
    assert_eq!(profile.specialization, String::from_str(&e, specialization));
    assert_eq!(profile.institution_wallet, institution);
    assert_eq!(profile.metadata, String::from_str(&e, ""));
    
    // Verify timestamps
    let current_time = e.ledger().timestamp();
    assert_eq!(profile.created_at, 1234567890);
    assert_eq!(profile.updated_at, 1234567890);
    assert_eq!(profile.created_at, current_time);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_create_duplicate_doctor_profile() {
    let e = Env::default();
    let contract = create_test_contract(&e);
    
    let wallet = Address::generate(&e);
    let institution = Address::generate(&e);

    // Create first profile
    create_test_doctor(
        &e,
        &contract,
        &wallet,
        "Dr. John Doe",
        "Cardiology",
        &institution,
    );

    // Attempt to create duplicate profile
    create_test_doctor(
        &e,
        &contract,
        &wallet,
        "Dr. Jane Doe",
        "Neurology",
        &institution,
    );
}

#[test]
fn test_update_doctor_profile() {
    let e = Env::default();
    let contract = create_test_contract(&e);
    
    let wallet = Address::generate(&e);
    let institution = Address::generate(&e);

    // Create initial profile
    create_test_doctor(
        &e,
        &contract,
        &wallet,
        "Dr. John Doe",
        "Cardiology",
        &institution,
    );

    // Update specialization
    let new_specialization = "Pediatric Cardiology";
    contract.update_doctor_profile(
        &wallet,
        &Some(String::from_str(&e, new_specialization)),
        &None,
    );

    // Verify update
    let profile = contract.get_doctor_profile(&wallet);
    assert_eq!(profile.specialization, String::from_str(&e, new_specialization));
    assert_eq!(profile.metadata, String::from_str(&e, ""));

    // Update metadata
    let new_metadata = "Board Certified";
    contract.update_doctor_profile(
        &wallet,
        &None,
        &Some(String::from_str(&e, new_metadata)),
    );

    // Verify metadata update
    let profile = contract.get_doctor_profile(&wallet);
    assert_eq!(profile.metadata, String::from_str(&e, new_metadata));
    assert_eq!(profile.specialization, String::from_str(&e, new_specialization));
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_update_nonexistent_doctor() {
    let e = Env::default();
    let contract = create_test_contract(&e);
    
    let wallet = Address::generate(&e);

    // Attempt to update non-existent profile
    contract.update_doctor_profile(
        &wallet,
        &Some(String::from_str(&e, "New Specialization")),
        &None,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_get_nonexistent_doctor() {
    let e = Env::default();
    let contract = create_test_contract(&e);
    
    let wallet = Address::generate(&e);

    // Attempt to get non-existent profile
    contract.get_doctor_profile(&wallet);
}

#[test]
fn test_multiple_doctors() {
    let e = Env::default();
    let contract = create_test_contract(&e);
    
    let institution = Address::generate(&e);
    let doctor1 = Address::generate(&e);
    let doctor2 = Address::generate(&e);

    // Create first doctor
    create_test_doctor(
        &e,
        &contract,
        &doctor1,
        "Dr. John Doe",
        "Cardiology",
        &institution,
    );

    // Create second doctor
    create_test_doctor(
        &e,
        &contract,
        &doctor2,
        "Dr. Jane Smith",
        "Neurology",
        &institution,
    );

    // Verify both profiles exist and are correct
    let profile1 = contract.get_doctor_profile(&doctor1);
    let profile2 = contract.get_doctor_profile(&doctor2);

    assert_eq!(profile1.name, String::from_str(&e, "Dr. John Doe"));
    assert_eq!(profile2.name, String::from_str(&e, "Dr. Jane Smith"));
    assert_eq!(profile1.specialization, String::from_str(&e, "Cardiology"));
    assert_eq!(profile2.specialization, String::from_str(&e, "Neurology"));
}

#[test]
fn test_update_timestamps() {
    let e = Env::default();
    let contract = create_test_contract(&e);
    
    let wallet = Address::generate(&e);
    let institution = Address::generate(&e);

    // Create initial profile
    create_test_doctor(
        &e,
        &contract,
        &wallet,
        "Dr. John Doe",
        "Cardiology",
        &institution,
    );

    // Get initial timestamps
    let initial_profile = contract.get_doctor_profile(&wallet);
    let initial_created_at = initial_profile.created_at;
    let initial_updated_at = initial_profile.updated_at;

    // Advance time
    e.ledger().with_mut(|l| {
        l.timestamp = 1234567891;
    });

    // Update profile
    contract.update_doctor_profile(
        &wallet,
        &Some(String::from_str(&e, "New Specialization")),
        &None,
    );

    // Verify timestamps
    let updated_profile = contract.get_doctor_profile(&wallet);
    assert_eq!(updated_profile.created_at, initial_created_at);
    assert!(updated_profile.updated_at > initial_updated_at);
    assert_eq!(updated_profile.updated_at, 1234567891);
}

#[test]
fn test_empty_string_updates() {
    let e = Env::default();
    let contract = create_test_contract(&e);
    
    let wallet = Address::generate(&e);
    let institution = Address::generate(&e);

    // Create initial profile
    create_test_doctor(
        &e,
        &contract,
        &wallet,
        "Dr. John Doe",
        "Cardiology",
        &institution,
    );

    // Update with empty strings
    contract.update_doctor_profile(
        &wallet,
        &Some(String::from_str(&e, "")),
        &Some(String::from_str(&e, "")),
    );

    // Verify updates
    let profile = contract.get_doctor_profile(&wallet);
    assert_eq!(profile.specialization, String::from_str(&e, ""));
    assert_eq!(profile.metadata, String::from_str(&e, ""));
}

#[test]
fn test_same_institution_multiple_doctors() {
    let e = Env::default();
    let contract = create_test_contract(&e);
    
    let institution = Address::generate(&e);
    let mut doctors = Vec::new(&e);
    
    // Create multiple doctors for the same institution
    let names = ["Dr. John", "Dr. Jane", "Dr. Bob", "Dr. Alice", "Dr. Charlie"];
    let specializations = ["Cardiology", "Neurology", "Pediatrics", "Dermatology", "Orthopedics"];
    
    for i in 0..5 {
        let doctor = Address::generate(&e);
        doctors.push_back(doctor.clone());
        
        create_test_doctor(
            &e,
            &contract,
            &doctor,
            names[i],
            specializations[i],
            &institution,
        );
    }

    // Verify all doctors are associated with the same institution
    for doctor in doctors.iter() {
        let profile = contract.get_doctor_profile(&doctor);
        assert_eq!(profile.institution_wallet, institution);
    }
}

#[test]
fn test_metadata_persistence() {
    let e = Env::default();
    let contract = create_test_contract(&e);
    
    let wallet = Address::generate(&e);
    let institution = Address::generate(&e);

    // Create profile with initial metadata
    create_test_doctor(
        &e,
        &contract,
        &wallet,
        "Dr. John Doe",
        "Cardiology",
        &institution,
    );

    // Update metadata multiple times
    let metadata_values = [
        "Board Certified",
        "Fellowship",
        "Research Award"
    ];
    
    for metadata in metadata_values.iter() {
        contract.update_doctor_profile(
            &wallet,
            &None,
            &Some(String::from_str(&e, metadata)),
        );

        let profile = contract.get_doctor_profile(&wallet);
        assert_eq!(profile.metadata, String::from_str(&e, metadata));
    }
} 