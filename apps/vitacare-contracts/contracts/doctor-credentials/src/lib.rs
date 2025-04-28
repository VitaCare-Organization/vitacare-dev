#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Bytes, String, Vec};

#[contract]
pub struct DoctorCredentials;

#[contractimpl]
impl DoctorCredentials {
    // Register a new doctor with their credentials
    pub fn register_doctor(
        env: soroban_sdk::Env,
        wallet: Address,
        name: String,
        specialization: String,
        certificate_hash: Bytes,
    ) -> Result<(), String> {
        // Check if doctor already exists
        if env.storage().instance().has(&DataKey::Doctor(wallet.clone())) {
            return Err(String::from_str(&env, "Doctor already registered"));
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
        env: soroban_sdk::Env,
        doctor_wallet: Address,
        institution_wallet: Address,
    ) -> Result<(), String> {
        // Check if institution is verified
        if !env.storage().instance().has(&DataKey::Institution(institution_wallet.clone())) {
            return Err(String::from_str(&env, "Institution not verified"));
        }

        // Get doctor data
        let mut doctor_data: DoctorData = env
            .storage()
            .instance()
            .get(&DataKey::Doctor(doctor_wallet.clone()))
            .ok_or_else(|| String::from_str(&env, "Doctor not found"))?;

        // Update verification status
        doctor_data.is_verified = true;
        doctor_data.verified_by = Some(institution_wallet);

        // Store updated doctor data
        env.storage().instance().set(&DataKey::Doctor(doctor_wallet), &doctor_data);
        Ok(())
    }

    // Get doctor data
    pub fn get_doctor(env: soroban_sdk::Env, wallet: Address) -> Result<DoctorData, String> {
        env.storage()
            .instance()
            .get(&DataKey::Doctor(wallet))
            .ok_or_else(|| String::from_str(&env, "Doctor not found"))
    }

    // Add a verified medical institution
    pub fn add_institution(env: soroban_sdk::Env, institution: Address) -> Result<(), String> {
        env.storage().instance().set(&DataKey::Institution(institution), &true);
        Ok(())
    }
}

// Data structures
#[derive(Clone, soroban_sdk::Serialize, soroban_sdk::Deserialize)]
pub struct DoctorData {
    pub name: String,
    pub specialization: String,
    pub certificate_hash: Bytes,
    pub is_verified: bool,
    pub verified_by: Option<Address>,
}

// Storage keys
#[derive(Clone, soroban_sdk::Serialize, soroban_sdk::Deserialize)]
enum DataKey {
    Doctor(Address),
    Institution(Address),
}

// Test module
#[cfg(test)]
mod test; 