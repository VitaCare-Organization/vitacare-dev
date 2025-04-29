#![no_std]
use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Env, String
};

mod types;
mod error;
mod storage;

use types::InstitutionData;
use error::Error;
use storage::{get_institutions, save_institutions, get_admin, save_admin};

#[contract]
pub struct InstitutionRegistry;

#[contractimpl]
impl InstitutionRegistry {
    /// Register a new medical institution
    ///
    /// # Arguments
    /// * `env` - The environment the contract is executing within
    /// * `wallet` - The wallet address of the institution
    /// * `name` - The name of the institution
    /// * `license_id` - The license ID of the institution
    /// * `metadata` - Additional metadata about the institution (JSON string)
    ///
    /// # Returns
    /// * `InstitutionData` - The data of the registered institution
    ///
    /// # Errors
    /// * Panics if the institution is already registered
    pub fn register_institution(
        env: Env, 
        wallet: Address, 
        name: String, 
        license_id: String, 
        metadata: String
    ) -> InstitutionData {
        // Verify transaction is signed by the wallet being registered
        wallet.require_auth();
        
        // Get existing institutions
        let institutions = get_institutions(&env);
        
        // Check if institution already exists
        if institutions.contains_key(wallet.clone()) {
            env.panic_with_error(Error::InstitutionAlreadyRegistered);
        }
        
        // Create institution data
        let institution_data = InstitutionData {
            name,
            license_id,
            metadata,
            verified: false,
        };
        
        // Store institution data
        let mut updated_institutions = institutions.clone();
        updated_institutions.set(wallet.clone(), institution_data.clone());
        save_institutions(&env, &updated_institutions);
        
        // Return the institution data
        institution_data
    }
    
    /// Get institution data
    ///
    /// # Arguments
    /// * `env` - The environment the contract is executing within
    /// * `wallet` - The wallet address of the institution
    ///
    /// # Returns
    /// * `InstitutionData` - The data of the institution
    ///
    /// # Errors
    /// * Panics if the institution is not found
    pub fn get_institution(env: Env, wallet: Address) -> InstitutionData {
        // Get existing institutions
        let institutions = get_institutions(&env);
        
        // Check if institution exists
        if !institutions.contains_key(wallet.clone()) {
            env.panic_with_error(Error::InstitutionNotFound);
        }
        
        // Return institution data
        institutions.get(wallet).unwrap().clone()
    }
    
    /// Update institution metadata
    ///
    /// # Arguments
    /// * `env` - The environment the contract is executing within
    /// * `wallet` - The wallet address of the institution
    /// * `metadata` - New metadata for the institution (JSON string)
    ///
    /// # Returns
    /// * `InstitutionData` - The updated data of the institution
    ///
    /// # Errors
    /// * Panics if the institution is not found
    pub fn update_institution(env: Env, wallet: Address, metadata: String) -> InstitutionData {
        // Verify transaction is signed by the wallet being updated
        wallet.require_auth();
        
        // Get existing institutions
        let institutions = get_institutions(&env);
        
        // Check if institution exists
        if !institutions.contains_key(wallet.clone()) {
            env.panic_with_error(Error::InstitutionNotFound);
        }
        
        // Get current data and update metadata
        let mut institution_data = institutions.get(wallet.clone()).unwrap().clone();
        institution_data.metadata = metadata;
        
        // Store updated institution data
        let mut updated_institutions = institutions.clone();
        updated_institutions.set(wallet.clone(), institution_data.clone());
        save_institutions(&env, &updated_institutions);
        
        // Return updated institution data
        institution_data
    }
    
    /// Verify an institution (only callable by admin)
    ///
    /// # Arguments
    /// * `env` - The environment the contract is executing within
    /// * `admin` - The admin address performing the verification
    /// * `wallet` - The wallet address of the institution to verify
    ///
    /// # Returns
    /// * `InstitutionData` - The updated data of the institution
    ///
    /// # Errors
    /// * Panics if the caller is not an admin
    /// * Panics if the institution is not found
    pub fn verify_institution(env: Env, admin: Address, wallet: Address) -> InstitutionData {
        // Verify transaction is signed by the admin
        admin.require_auth();
        
        // Check if admin is authorized
        let is_admin = env.storage().instance().has(&symbol_short!("admin"))
            && get_admin(&env) == admin;
            
        if !is_admin {
            env.panic_with_error(Error::Unauthorized);
        }
        
        // Get existing institutions
        let institutions = get_institutions(&env);
        
        // Check if institution exists
        if !institutions.contains_key(wallet.clone()) {
            env.panic_with_error(Error::InstitutionNotFound);
        }
        
        // Get current data and mark as verified
        let mut institution_data = institutions.get(wallet.clone()).unwrap().clone();
        institution_data.verified = true;
        
        // Store updated institution data
        let mut updated_institutions = institutions.clone();
        updated_institutions.set(wallet.clone(), institution_data.clone());
        save_institutions(&env, &updated_institutions);
        
        // Return updated institution data
        institution_data
    }
    
    /// Set the admin address (only callable once during initialization or by current admin)
    ///
    /// # Arguments
    /// * `env` - The environment the contract is executing within
    /// * `admin` - The new admin address
    ///
    /// # Errors
    /// * Panics if the caller is not the current admin (if one is set)
    pub fn set_admin(env: Env, admin: Address) {
        // If admin is already set, require auth from the current admin
        if env.storage().instance().has(&symbol_short!("admin")) {
            let current_admin = get_admin(&env);
            current_admin.require_auth();
        }
        
        // Set the new admin
        save_admin(&env, &admin);
    }
}

mod test; 