use soroban_sdk::{Address, Env, String, log};
use crate::storage::{get_record, get_patient_record_ids, save_record};
use crate::access::{check_doctor_authorization, grant_doctor_access, revoke_doctor_access, get_all_authorized_doctors};
use crate::types::{MedicalRecord, RecordId, PatientRecordIds};

/// Agrega un nuevo registro médico
pub fn add_record(
    env: &Env,
    patient: &Address,
    doctor: &Address,
    notes: &String,
    diagnosis: &String,
    treatment: &String,
) -> RecordId {
    // Verificar que el doctor está autorizado para este paciente
    if !check_doctor_authorization(env, patient, doctor) {
        log!(env, "Doctor not authorized for patient");
        panic!("Doctor not authorized for this patient");
    }

    // Guardar el registro
    let timestamp = env.ledger().timestamp();
    let id = save_record(
        env,
        patient.clone(),
        doctor.clone(),
        timestamp,
        notes.clone(),
        diagnosis.clone(),
        treatment.clone(),
    );

    log!(env, "Record added with ID: {}", id);
    id
}

/// Obtiene un registro médico específico
pub fn get_medical_record(
    env: &Env,
    patient: &Address,
    id: &RecordId,
) -> MedicalRecord {
    get_record(env, patient, id)
}

/// Obtiene todos los IDs de registros para un paciente
pub fn get_patient_records(
    env: &Env,
    patient: &Address,
) -> PatientRecordIds {
    get_patient_record_ids(env, patient)
}

/// Concede acceso a un doctor
pub fn authorize_doctor(
    env: &Env,
    patient: &Address,
    doctor: &Address,
) {
    // Verificar que el llamante es el paciente
    let caller = env.invoker();
    if caller != *patient {
        log!(env, "Unauthorized caller: {}", caller);
        panic!("Only the patient can grant access to doctors");
    }

    grant_doctor_access(env, patient, doctor);
    log!(env, "Access granted to doctor {} for patient {}", doctor, patient);
}

/// Revoca acceso a un doctor
pub fn unauthorize_doctor(
    env: &Env,
    patient: &Address,
    doctor: &Address,
) {
    // Verificar que el llamante es el paciente
    let caller = env.invoker();
    if caller != *patient {
        log!(env, "Unauthorized caller: {}", caller);
        panic!("Only the patient can revoke access from doctors");
    }

    revoke_doctor_access(env, patient, doctor);
    log!(env, "Access revoked from doctor {} for patient {}", doctor, patient);
}

/// Obtiene todos los doctores autorizados para un paciente
pub fn get_authorized_doctors(
    env: &Env,
    patient: &Address,
) -> soroban_sdk::Vec<Address> {
    get_all_authorized_doctors(env, patient)
} 