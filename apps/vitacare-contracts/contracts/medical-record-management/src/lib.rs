#![no_std]

mod types;
mod error;
mod storage;
mod records;
mod access;
mod test; 

use soroban_sdk::{contract, contractimpl, Env, String, Vec, Address, BytesN, log};
use types::RecordMetadata;

#[contract]
pub struct MedicalRecordsContract;

#[contractimpl]
impl MedicalRecordsContract {
    /// Add a new medical record for a patient
    pub fn add_medical_record(
        env: Env,
        patient: Address,
        doctor: Address,
        record_hash: BytesN<32>,
        description: String,
    ) -> Result<(), error::Error> {
        log!(&env, "add_medical_record: Verificando autorización para doctor: {:?}", doctor);
        
        // Verificar si el doctor está autorizado
        if !access::check_doctor_authorization(&env, &patient, &doctor) {
            log!(&env, "add_medical_record: Doctor no autorizado");
            return Err(error::Error::Unauthorized);
        }
        
        log!(&env, "add_medical_record: Doctor autorizado, añadiendo registro");
        records::add_record(&env, &patient, &doctor, &record_hash, &description)
    }

    /// Get all medical records for a patient
    pub fn get_medical_records(env: Env, patient: Address) -> Result<Vec<RecordMetadata>, error::Error> {
        log!(&env, "get_medical_records: Obteniendo registros para paciente: {:?}", patient);
        records::get_patient_records(&env, &patient)
    }

    /// Grant access to a doctor
    pub fn grant_access(env: Env, patient: Address, doctor: Address) -> Result<(), error::Error> {
        log!(&env, "grant_access: Autorizando al paciente: {:?}", patient);
        patient.require_auth();
        log!(&env, "grant_access: Autorizando al doctor: {:?}", doctor);
        access::grant_doctor_access(&env, &patient, &doctor)
    }

    /// Revoke access from a doctor
    pub fn revoke_access(env: Env, patient: Address, doctor: Address) -> Result<(), error::Error> {
        log!(&env, "revoke_access: Verificando autorización del paciente: {:?}", patient);
        patient.require_auth();
        log!(&env, "revoke_access: Revocando acceso al doctor: {:?}", doctor);
        access::revoke_doctor_access(&env, &patient, &doctor)
    }

    /// Get all doctors authorized for a patient
    pub fn get_authorized_doctors(env: Env, patient: Address) -> Result<Vec<Address>, error::Error> {
        log!(&env, "get_authorized_doctors: Obteniendo doctores para paciente: {:?}", patient);
        Ok(access::get_all_authorized_doctors(&env, &patient))
    }
}