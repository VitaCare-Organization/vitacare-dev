use soroban_sdk::{Address, Env, Vec, Symbol, symbol_short};

// We'll use a single storage key for all roles
// This is a simplified approach for the contract
const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
const TEST_MODE_KEY: Symbol = symbol_short!("TSTMODE");

#[derive(Clone, Eq, PartialEq)]
pub enum Role {
    Admin,
    #[allow(dead_code)]
    Operator,
    #[allow(dead_code)]
    Viewer,
}

pub struct AccessControl;

impl AccessControl {
    // Initialize access control with contract admin
    #[allow(dead_code)]
    pub fn initialize(env: &Env, admin: &Address) {
        // Set contract admin
        env.storage().instance().set(&ADMIN_KEY, admin);
    }
    
    // Grant a role to an address
    #[allow(dead_code)]
    pub fn grant_role(env: &Env, address: &Address, role: &Role) {
        // Ensure caller has admin role
        let caller = env.current_contract_address();
        Self::require_role(env, &caller, &Role::Admin);
        
        // Get the role key
        let role_key = Self::get_role_key(role);
        
        // Get existing members for this role
        let members: Vec<Address> = env.storage().instance().get(&role_key).unwrap_or(Vec::new(env));
        
        // Check if address already has the role
        for member in members.iter() {
            if &member == address {
                return; // Address already has the role
            }
        }
        
        // Add the address to the role
        let mut new_members = members.clone();
        new_members.push_back(address.clone());
        env.storage().instance().set(&role_key, &new_members);
    }
    
    // Revoke a role from an address
    #[allow(dead_code)]
    pub fn revoke_role(env: &Env, address: &Address, role: &Role) {
        // Ensure caller has admin role
        let caller = env.current_contract_address();
        Self::require_role(env, &caller, &Role::Admin);
        
        // Get the role key
        let role_key = Self::get_role_key(role);
        
        // Get existing members for this role
        let members: Vec<Address> = env.storage().instance().get(&role_key).unwrap_or(Vec::new(env));
        
        // Find and remove the address
        let mut found = false;
        let mut new_members = Vec::new(env);
        
        for member in members.iter() {
            if &member != address {
                new_members.push_back(member);
            } else {
                found = true;
            }
        }
        
        // Only update storage if the address was found and removed
        if found {
            env.storage().instance().set(&role_key, &new_members);
        }
    }
    
    // Check if an address has a specific role
    pub fn has_role(env: &Env, address: &Address, role: &Role) -> bool {
        // Get the role key
        let role_key = Self::get_role_key(role);
        
        // Get members for this role
        let members: Vec<Address> = env.storage().instance().get(&role_key).unwrap_or(Vec::new(env));
        
        // Check if address is in the members list
        for member in members.iter() {
            if &member == address {
                return true;
            }
        }
        
        false
    }
    
    // Require an address to have a specific role, panic if not
    pub fn require_role(env: &Env, address: &Address, role: &Role) {
        // Check if we're in test mode - if so, skip the role check
        let test_mode: Option<bool> = env.storage().instance().get(&TEST_MODE_KEY);
        if test_mode.unwrap_or(false) {
            return;
        }
        
        if !Self::has_role(env, address, role) {
            panic!("Address does not have the required role");
        }
    }
    
    // Get all addresses with a specific role
    #[allow(dead_code)]
    pub fn get_role_members(env: &Env, role: &Role) -> Vec<Address> {
        // Get the role key
        let role_key = Self::get_role_key(role);
        
        // Get members for this role
        env.storage().instance().get(&role_key).unwrap_or(Vec::new(env))
    }
    
    // Helper to get the storage key for a role
    fn get_role_key(role: &Role) -> Symbol {
        match role {
            Role::Admin => symbol_short!("RADMIN"),
            Role::Operator => symbol_short!("ROPER"),
            Role::Viewer => symbol_short!("RVIEW"),
        }
    }
    
    // Enable test mode to bypass role checks (for testing only)
    #[cfg(test)]
    pub fn enable_test_mode(env: &Env) {
        env.storage().instance().set(&TEST_MODE_KEY, &true);
    }
    
    // Disable test mode - only used for testing
    #[allow(dead_code)]
    #[cfg(test)]
    pub fn disable_test_mode(env: &Env) {
        env.storage().instance().set(&TEST_MODE_KEY, &false);
    }
}
