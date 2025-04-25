use soroban_sdk::{Env, Address, Vec};
use crate::storage;
use crate::error::Error;

/// Checks if a doctor is authorized to access a patient's records
pub fn check_doctor_authorization(
    env: &Env,
    patient: &Address,
    doctor: &Address,
) -> bool {
    // If the doctor is the patient, they are always authorized
    if doctor == patient {
        return true;
    }
    
    // Check in the list of authorized doctors
    let doctors = storage::get_authorized_doctors(env, patient);
    
    for auth_doctor in doctors.iter() {
        if &auth_doctor == doctor {
            return true;
        }
    }
    
    false
}

/// Grants access to a doctor to view a patient's medical records
pub fn grant_doctor_access(
    env: &Env,
    patient: &Address,
    doctor: &Address,
) -> Result<(), Error> {
    // Get the current list of authorized doctors
    let mut doctors = storage::get_authorized_doctors(env, patient);
    
    // Check if the doctor is already authorized
    for auth_doctor in doctors.iter() {
        if &auth_doctor == doctor {
            // Doctor is already authorized
            return Ok(());
        }
    }
    
    // Add the new doctor to the list
    doctors.push_back(doctor.clone());
    
    // Save the updated list
    storage::save_authorized_doctors(env, patient, &doctors);
    
    Ok(())
}

/// Revokes a doctor's access to a patient's medical records
pub fn revoke_doctor_access(
    env: &Env,
    patient: &Address,
    doctor: &Address,
) -> Result<(), Error> {
    // Get the current list of authorized doctors
    let current_doctors = storage::get_authorized_doctors(env, patient);
    
    // Create a new list without the doctor being revoked
    let mut updated_doctors = Vec::new(env);
    for auth_doctor in current_doctors.iter() {
        if &auth_doctor != doctor {
            updated_doctors.push_back(auth_doctor);
        }
    }
    
    // Save the updated list
    storage::save_authorized_doctors(env, patient, &updated_doctors);
    
    Ok(())
}

/// Gets all authorized doctors for a patient
pub fn get_all_authorized_doctors(
    env: &Env,
    patient: &Address,
) -> Vec<Address> {
    storage::get_authorized_doctors(env, patient)
} 