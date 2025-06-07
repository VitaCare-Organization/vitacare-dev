// SPDX-License-Identifier: MIT
use soroban_sdk::{symbol_short, Address, Env, Symbol};

use crate::types::{PatientData, PatientRegistry};

pub const REGISTRY: Symbol = symbol_short!("REGISTRY");
pub const PATIENT_PREFIX: Symbol = symbol_short!("PATIENT");

pub struct Storage;

impl Storage {
    pub fn get_registry(env: &Env) -> Option<PatientRegistry> {
        env.storage().instance().get(&REGISTRY)
    }

    pub fn set_registry(env: &Env, registry: &PatientRegistry) {
        env.storage().instance().set(&REGISTRY, registry);
    }

    pub fn get_patient(env: &Env, wallet: &Address) -> Option<PatientData> {
        let patient_key = Self::patient_storage_key(env, wallet);
        env.storage().instance().get(&patient_key)
    }

    pub fn set_patient(env: &Env, wallet: &Address, patient_data: &PatientData) {
        let patient_key = Self::patient_storage_key(env, wallet);
        env.storage().instance().set(&patient_key, patient_data);
    }

    pub fn has_patient(env: &Env, wallet: &Address) -> bool {
        let patient_key = Self::patient_storage_key(env, wallet);
        env.storage().instance().has(&patient_key)
    }

    pub fn has_registry(env: &Env) -> bool {
        env.storage().instance().has(&REGISTRY)
    }

    /// Generate a unique storage key for each patient
    pub fn patient_storage_key(env: &Env, wallet: &Address) -> Symbol {
        Symbol::new(
            env,
            &format!("{}_{}", PATIENT_PREFIX.to_string(), wallet.to_string()),
        )
    }
}