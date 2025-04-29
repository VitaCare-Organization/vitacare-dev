use soroban_sdk::{Address, Env, Map, symbol_short};
use crate::types::InstitutionData;

/// Storage key for the institutions map
const INSTITUTIONS_KEY: &[u8] = b"inst";

/// Storage key for the admin address
const ADMIN_KEY: &[u8] = b"admin";

/// Retrieves the institutions map from storage
///
/// # Arguments
/// * `env` - The environment the contract is executing within
///
/// # Returns
/// * A map of institution addresses to their data
pub fn get_institutions(env: &Env) -> Map<Address, InstitutionData> {
    let key = symbol_short!("inst");
    env.storage().instance().get::<_, Map<Address, InstitutionData>>(&key)
        .unwrap_or_else(|| Map::new(env))
}

/// Saves the institutions map to storage
///
/// # Arguments
/// * `env` - The environment the contract is executing within
/// * `institutions` - The institutions map to save
pub fn save_institutions(env: &Env, institutions: &Map<Address, InstitutionData>) {
    let key = symbol_short!("inst");
    env.storage().instance().set(&key, institutions);
}

/// Retrieves the admin address from storage
///
/// # Arguments
/// * `env` - The environment the contract is executing within
///
/// # Returns
/// * The admin address
pub fn get_admin(env: &Env) -> Address {
    let key = symbol_short!("admin");
    env.storage().instance().get::<_, Address>(&key).unwrap()
}

/// Saves the admin address to storage
///
/// # Arguments
/// * `env` - The environment the contract is executing within
/// * `admin` - The admin address to save
pub fn save_admin(env: &Env, admin: &Address) {
    let key = symbol_short!("admin");
    env.storage().instance().set(&key, admin);
} 