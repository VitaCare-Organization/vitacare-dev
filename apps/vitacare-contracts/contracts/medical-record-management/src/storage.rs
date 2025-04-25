use soroban_sdk::{Env, Vec, Address, Map, symbol_short};
use crate::types::{RecordMetadata};

/// Initialize and get the records map for a patient
fn get_records_map(env: &Env) -> Map<Address, Vec<RecordMetadata>> {
    let key = symbol_short!("records");
    
    if !env.storage().instance().has(&key) {
        let map: Map<Address, Vec<RecordMetadata>> = Map::new(env);
        env.storage().instance().set(&key, &map);
    }
    
    env.storage().instance().get(&key).unwrap()
}

/// Initialize and get the authorized doctors map
fn get_doctors_map(env: &Env) -> Map<Address, Vec<Address>> {
    let key = symbol_short!("doctors");
    
    if !env.storage().instance().has(&key) {
        let map: Map<Address, Vec<Address>> = Map::new(env);
        env.storage().instance().set(&key, &map);
    }
    
    env.storage().instance().get(&key).unwrap()
}

/// Initialize and get the record counters map
fn get_counters_map(env: &Env) -> Map<Address, u32> {
    let key = symbol_short!("counters");
    
    if !env.storage().instance().has(&key) {
        let map: Map<Address, u32> = Map::new(env);
        env.storage().instance().set(&key, &map);
    }
    
    env.storage().instance().get(&key).unwrap()
}

/// Increment and get the next ID for a medical record of a specific patient
pub fn increment_record_id_counter(env: &Env, patient: &Address) -> u32 {
    let mut counters = get_counters_map(env);
    
    // Retrieve current counter or initialize to 0
    let current_counter: u32 = if counters.contains_key(patient.clone()) {
        counters.get(patient.clone()).unwrap()
    } else {
        0
    };
    
    // Increment counter
    let next_id = current_counter + 1;
    counters.set(patient.clone(), next_id);
    
    // Save the updated map
    let key = symbol_short!("counters");
    env.storage().instance().set(&key, &counters);
    
    next_id
}

/// Save a new medical record
pub fn save_record(
    env: &Env,
    patient: &Address,
    record: &RecordMetadata,
) {
    let mut records_map = get_records_map(env);
    
    // Get the patient's records list or create a new one
    let mut patient_records: Vec<RecordMetadata> = if records_map.contains_key(patient.clone()) {
        records_map.get(patient.clone()).unwrap()
    } else {
        Vec::new(env)
    };
    
    // Add the new record
    patient_records.push_back(record.clone());
    
    // Update the map
    records_map.set(patient.clone(), patient_records);
    
    // Save the updated map
    let key = symbol_short!("records");
    env.storage().instance().set(&key, &records_map);
}

/// Get a specific medical record by ID
pub fn get_record(
    env: &Env,
    patient: &Address,
    id: &u32,
) -> Option<RecordMetadata> {
    let records_map = get_records_map(env);
    
    if !records_map.contains_key(patient.clone()) {
        return None;
    }
    
    let patient_records = records_map.get(patient.clone()).unwrap();
    
    for rec in patient_records.iter() {
        if rec.id == *id {
            return Some(rec);
        }
    }
    
    None
}

/// Get all record IDs for a patient
pub fn get_patient_record_ids(
    env: &Env,
    patient: &Address,
) -> Vec<u32> {
    let records_map = get_records_map(env);
    
    if !records_map.contains_key(patient.clone()) {
        return Vec::new(env);
    }
    
    let patient_records = records_map.get(patient.clone()).unwrap();
    let mut ids = Vec::new(env);
    
    for rec in patient_records.iter() {
        ids.push_back(rec.id);
    }
    
    ids
}

/// Save the list of authorized doctors
pub fn save_authorized_doctors(
    env: &Env,
    patient: &Address,
    doctors: &Vec<Address>,
) {
    let mut doctors_map = get_doctors_map(env);
    
    // Update the doctors list
    doctors_map.set(patient.clone(), doctors.clone());
    
    // Save the updated map
    let key = symbol_short!("doctors");
    env.storage().instance().set(&key, &doctors_map);
}

/// Get the list of authorized doctors
pub fn get_authorized_doctors(
    env: &Env,
    patient: &Address,
) -> Vec<Address> {
    let doctors_map = get_doctors_map(env);
    
    if !doctors_map.contains_key(patient.clone()) {
        return Vec::new(env);
    }
    
    doctors_map.get(patient.clone()).unwrap()
} 