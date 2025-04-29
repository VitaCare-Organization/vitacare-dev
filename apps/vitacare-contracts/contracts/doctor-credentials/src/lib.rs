#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, String};

mod types;
use types::{ContractError, DataKey, DoctorData};

#[contract]
pub struct DoctorCredentials;

#[contractimpl]
impl DoctorCredentials {
    // Register a new doctor with their credentials
    pub fn register_doctor(
        env: Env,
        wallet: Address,
        name: String,
        specialization: String,
        certificate_hash: Bytes,
    ) -> Result<(), ContractError> {
        // Validate inputs
        if name.is_empty() || specialization.is_empty() || certificate_hash.len() == 0 {
            return Err(ContractError::InvalidInput);
        }

        // Check if doctor already exists
        if env.storage().instance().has(&DataKey::Doctor(wallet.clone())) {
            return Err(ContractError::DoctorAlreadyExists);
        }

        // Create new doctor data
        let doctor_data = DoctorData {
            name,
            specialization,
            certificate_hash,
            is_verified: false,
            verified_by: None,
        };

        // Store doctor data
        env.storage().instance().set(&DataKey::Doctor(wallet), &doctor_data);
        Ok(())
    }

    // Verify a doctor by a medical institution
    pub fn verify_doctor(
        env: Env,
        doctor_wallet: Address,
        institution_wallet: Address,
    ) -> Result<(), ContractError> {
        // Check if institution is verified
        if !env.storage().instance().has(&DataKey::Institution(institution_wallet.clone())) {
            return Err(ContractError::InstitutionNotVerified);
        }

        // Get doctor data
        let mut doctor_data: DoctorData = env
            .storage()
            .instance()
            .get(&DataKey::Doctor(doctor_wallet.clone()))
            .ok_or(ContractError::DoctorNotFound)?;

        // Update verification status
        doctor_data.is_verified = true;
        doctor_data.verified_by = Some(institution_wallet);

        // Store updated doctor data
        env.storage().instance().set(&DataKey::Doctor(doctor_wallet), &doctor_data);
        Ok(())
    }

    // Get doctor data
    pub fn get_doctor(env: Env, wallet: Address) -> Result<DoctorData, ContractError> {
        env.storage()
            .instance()
            .get(&DataKey::Doctor(wallet))
            .ok_or(ContractError::DoctorNotFound)
    }

    // Add a verified medical institution
    pub fn add_institution(env: Env, institution: Address) -> Result<(), ContractError> {
        env.storage().instance().set(&DataKey::Institution(institution), &true);
        Ok(())
    }
}

// Test module
#[cfg(test)]
mod test; 