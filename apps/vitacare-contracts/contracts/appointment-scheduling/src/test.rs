#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, Env, Map, 
};
use crate::storage_types::{Appointment, AppointmentStatus, DataKey};
fn create_env<'a>() -> Env {
    Env::default()
}

fn register_contract(env: &Env) -> Address {
    env.register_contract(None, AppointmentSchedulingContract)
}

fn setup_ledger_time(env: &Env, timestamp: u64) {
    env.ledger().set(LedgerInfo {
        timestamp,
        protocol_version: 22,
        sequence_number: env.ledger().sequence(),
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 1_000_000,
        min_persistent_entry_ttl: 1_000_000,
        max_entry_ttl: 6_312_000,
    });
}

fn create_client<'a>(env: &'a Env, contract_id: &'a Address) -> contract::AppointmentSchedulingContractClient<'a> {
    contract::AppointmentSchedulingContractClient::new(env, contract_id)
}

#[test]
fn test_initialize_success() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);
    client.initialize();

    env.as_contract(&contract_id, || {
        // verify next ID exists and is 1
        let next_id: u64 = env.storage().instance()
            .get::<DataKey, u64>(&DataKey::NextAppointmentId) 
            .expect("Option should be Some");
        assert_eq!(next_id, 1);

         // verify appointments map exists and is empty
        let appointments: Map<u64, Appointment> = env.storage().persistent()
            .get::<DataKey, Map<u64, Appointment>>(&DataKey::Appointments) 
            .expect("Option should be Some");
        assert_eq!(appointments.len(), 0);
    });
}

#[test]
fn test_create_appointment_success() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let patient = Address::generate(&env);
    let doctor = Address::generate(&env);
    let appointment_time: u64 = 1746000000;
    setup_ledger_time(&env, appointment_time - 1000);

    // Call create_appointment directly - should not panic and return ID 1
    let appointment_id = client.create_appointment(&patient, &doctor, &appointment_time);
    assert_eq!(appointment_id, 1);

    // Verify appointment exists and has correct status using get_appointment_details
    let appointment = client.get_appointment_details(&appointment_id);
    assert_eq!(appointment.id, 1);
    assert_eq!(appointment.patient, patient);
    assert_eq!(appointment.doctor, doctor);
    assert_eq!(appointment.datetime, appointment_time);
    assert_eq!(appointment.status, AppointmentStatus::Scheduled);
}

#[test]
fn test_cancel_appointment_success() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let patient = Address::generate(&env);
    let doctor = Address::generate(&env);
    let appointment_time: u64 = 1746000000;
    setup_ledger_time(&env, appointment_time - 1000);

    // Setup: Create an appointment
    let appointment_id = client.create_appointment(&patient, &doctor, &appointment_time);
    assert_eq!(appointment_id, 1);

    // Test: Cancel the appointment directly - should not panic
    client.cancel_appointment(&appointment_id);

    // Verify: Check status is Canceled
    let appointment = client.get_appointment_details(&appointment_id);
    assert_eq!(appointment.status, AppointmentStatus::Canceled);
}

#[test]
fn test_complete_appointment_success() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let patient = Address::generate(&env);
    let doctor = Address::generate(&env);
    let appointment_time: u64 = 1746000000;
    setup_ledger_time(&env, appointment_time - 1000);

    // Setup: Create an appointment
    let appointment_id = client.create_appointment(&patient, &doctor, &appointment_time);
    assert_eq!(appointment_id, 1);

    // Test: Complete the appointment directly - should not panic
    client.complete_appointment(&appointment_id);

    // Verify: Check status is Completed
    let appointment = client.get_appointment_details(&appointment_id);
    assert_eq!(appointment.status, AppointmentStatus::Completed);
}

#[test]
fn test_get_appointment_details_success() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let patient = Address::generate(&env);
    let doctor = Address::generate(&env);
    let appointment_time: u64 = 1746000000;
    setup_ledger_time(&env, appointment_time - 1000);

    // Setup: Create an appointment
    let appointment_id = client.create_appointment(&patient, &doctor, &appointment_time);
    assert_eq!(appointment_id, 1);

    // Test: Get details directly - should not panic
    let appointment = client.get_appointment_details(&appointment_id);

    // Verify some details match
    assert_eq!(appointment.id, 1);
    assert_eq!(appointment.patient, patient);
    assert_eq!(appointment.doctor, doctor);
}

#[test]
fn test_get_appointments_success() {
    let env = create_env();
    env.mock_all_auths();
    let contract_id = register_contract(&env);
    let client = create_client(&env, &contract_id);

    client.initialize();

    let patient1 = Address::generate(&env);
    let doctor1 = Address::generate(&env);
    let patient2 = Address::generate(&env);
    let time1: u64 = 1746000000;
    let time2: u64 = 1746000100;
    setup_ledger_time(&env, time1 - 1000);

    // Setup: Create two appointments
    let id1 = client.create_appointment(&patient1, &doctor1, &time1);
    let id2 = client.create_appointment(&patient2, &doctor1, &time2); // Same doctor

    // Test: Get appointments for patient1 - should not panic
    let p1_appts = client.get_appointments(&patient1);
    assert_eq!(p1_appts.len(), 1);
    assert_eq!(p1_appts.get(0).expect("Index 0 out of bounds").id, id1);

     // Test: Get appointments for patient2 - should not panic
     let p2_appts = client.get_appointments(&patient2);
     assert_eq!(p2_appts.len(), 1);
     assert_eq!(p2_appts.get(0).expect("Index 0 out of bounds").id, id2);

    // Test: Get appointments for doctor1 - should not panic
    let d1_appts = client.get_appointments(&doctor1);
    assert_eq!(d1_appts.len(), 2);

    // Test: Get appointments for unrelated user - should not panic
    let other_user = Address::generate(&env);
    let other_appts = client.get_appointments(&other_user);
    assert_eq!(other_appts.len(), 0);
}

