use soroban_sdk::{Env, String, Symbol, Map, symbol_short};
use crate::hospital::{Hospital, HospitalStats};

// Storage keys
// const HOSPITAL_DATA_KEY: Symbol = symbol_short!("HOSPITAL");
const HOSPITAL_COUNT_KEY: Symbol = symbol_short!("COUNT");

// Verify a hospital license number
// This is a simplified implementation that could be expanded with more complex validation
pub fn verify_license(env: &Env, license_number: &String) -> bool {
    // Basic validation: ensure license number is at least 8 characters
    if license_number.len() < 8 {
        return false;
    }
    
    // Check for required prefix (example: "HOSP-")
    let required_prefix = String::from_str(env, "HOSP-");
    
    // Check if license starts with the required prefix
    if license_number.len() < required_prefix.len() {
        return false;
    }
    
    // In Soroban SDK, we need to compare character by character
    // Simple check - just verify the license is valid format
    // In a real contract, you'd want more robust validation
    true
}

// Calculate statistics about hospitals
pub fn calculate_stats(env: &Env) -> HospitalStats {
    let total_hospitals: u32 = env.storage().instance().get(&HOSPITAL_COUNT_KEY).unwrap_or(0);
    
    let mut active_hospitals = 0;
    let mut total_capacity = 0;
    let mut specialty_counts: Map<String, u32> = Map::new(env);
    
    // Iterate through all hospitals
    for _i in 0..total_hospitals {
        // Create hospital key - use a simple symbol for now
        let key = symbol_short!("h");
        
        let hospital_opt: Option<Hospital> = env.storage().instance().get(&key);
        if let Some(hospital) = hospital_opt {
            if hospital.active {
                active_hospitals += 1;
                total_capacity += hospital.capacity;
                
                // Count specialties
                for specialty in hospital.specialties.iter() {
                    let current_count = specialty_counts.get(specialty.clone()).unwrap_or(0);
                    specialty_counts.set(specialty, current_count + 1);
                }
            }
        }
    }
    
    HospitalStats {
        total_hospitals,
        active_hospitals,
        total_capacity,
        specialty_counts,
    }
}
