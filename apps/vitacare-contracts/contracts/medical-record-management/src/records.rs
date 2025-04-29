use soroban_sdk::{Env, Vec, Address, BytesN, String, log};
use crate::types::RecordMetadata;
use crate::error::Error;
use crate::storage;

/// Add a new medical record
pub fn add_record(
    env: &Env,
    patient: &Address,
    doctor: &Address,
    record_hash: &BytesN<32>,
    description: &String,
) -> Result<(), Error> {
    // Get new record ID from counter
    let record_id = storage::increment_record_id_counter(env, patient);
    
    // Create record metadata
    let record = RecordMetadata {
        id: record_id,
        patient: patient.clone(),
        doctor: doctor.clone(),
        data_id: record_hash.clone(),
        description: description.clone(),
    };
    
    // Save record to storage
    storage::save_record(env, patient, &record);
    
    Ok(())
}

/// Get all medical records for a patient
pub fn get_patient_records(env: &Env, patient: &Address) -> Result<Vec<RecordMetadata>, Error> {
    // Get record IDs for patient
    let record_ids = storage::get_patient_record_ids(env, patient);
    
    // Debug
    log!(env, "Record IDs for patient {:?}: {:?}", patient, record_ids);
    
    // Retrieve record metadata for each ID
    let mut records = Vec::new(env);
    for id in record_ids.iter() {
        if let Some(record) = storage::get_record(env, patient, &id) {
            // Verify that the record actually belongs to this patient
            if record.patient != *patient {
                log!(env, "WARNING! Record {:?} has patient {:?} but was requested by {:?}", 
                    id, record.patient, patient);
                continue; // Skip this record if it's not from the correct patient
            }
            records.push_back(record);
        } else {
            log!(env, "No record found with ID {:?} for patient {:?}", id, patient);
        }
    }
    
    Ok(records)
} 