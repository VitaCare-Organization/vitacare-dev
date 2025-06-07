#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, String, panic_with_error};

mod types;
mod storage;
mod errors;
mod test;

use types::DoctorProfile;
use storage::DoctorStorage;
use errors::DoctorError;

#[contract]
pub struct DoctorManagementContract;

#[contractimpl]
impl DoctorManagementContract {
    pub fn create_doctor_profile(
        env: &Env,
        wallet: Address,
        name: String,
        specialization: String,
        institution_wallet: Address,
    ) -> Result<(), DoctorError> {
        let storage = DoctorStorage::new(env);
        
        // Check if doctor profile already exists
        if storage.has_doctor(&wallet) {
            panic_with_error!(env, DoctorError::DoctorAlreadyExists);
        }

        let timestamp = env.ledger().timestamp();
        let profile = DoctorProfile {
            wallet: wallet.clone(),
            name,
            specialization,
            institution_wallet,
            metadata: String::from_str(env, ""),
            created_at: timestamp,
            updated_at: timestamp,
        };

        storage.save_doctor(&wallet, &profile);
        Ok(())
    }

    pub fn update_doctor_profile(
        env: &Env,
        wallet: Address,
        specialization: Option<String>,
        metadata: Option<String>,
    ) -> Result<(), DoctorError> {
        let storage = DoctorStorage::new(env);
        
        // Check if doctor profile exists
        if !storage.has_doctor(&wallet) {
            panic_with_error!(env, DoctorError::DoctorNotFound);
        }

        let mut profile = storage.get_doctor(&wallet)?;
        
        if let Some(spec) = specialization {
            profile.specialization = spec;
        }
        
        if let Some(meta) = metadata {
            profile.metadata = meta;
        }
        
        profile.updated_at = env.ledger().timestamp();
        storage.save_doctor(&wallet, &profile);
        
        Ok(())
    }

    pub fn get_doctor_profile(env: &Env, wallet: Address) -> Result<DoctorProfile, DoctorError> {
        let storage = DoctorStorage::new(env);
        if !storage.has_doctor(&wallet) {
            panic_with_error!(env, DoctorError::DoctorNotFound);
        }
        storage.get_doctor(&wallet)
    }
} 