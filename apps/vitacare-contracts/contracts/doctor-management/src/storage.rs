use soroban_sdk::{Env, Address, Symbol};

use crate::types::DoctorProfile;
use crate::errors::DoctorError;

pub struct DoctorStorage {
    env: Env,
}

impl DoctorStorage {
    pub fn new(env: &Env) -> Self {
        Self { env: env.clone() }
    }

    pub fn save_doctor(&self, wallet: &Address, profile: &DoctorProfile) {
        let key = self.get_storage_key(wallet);
        self.env.storage().persistent().set(&key, profile);
    }

    pub fn get_doctor(&self, wallet: &Address) -> Result<DoctorProfile, DoctorError> {
        let key = self.get_storage_key(wallet);
        self.env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(DoctorError::DoctorNotFound)
    }

    pub fn has_doctor(&self, wallet: &Address) -> bool {
        let key = self.get_storage_key(wallet);
        self.env.storage().persistent().has(&key)
    }

    fn get_storage_key(&self, wallet: &Address) -> (Symbol, Address) {
        (Symbol::new(&self.env, "DOCTOR"), wallet.clone())
    }
} 