use soroban_sdk::{contracttype, String};

/// InstitutionData struct to store information about medical institutions
#[contracttype]
#[derive(Clone)]
pub struct InstitutionData {
    /// Name of the medical institution
    pub name: String,
    
    /// License ID or registration number
    pub license_id: String,
    
    /// Additional metadata as a JSON string
    pub metadata: String,
    
    /// Whether the institution has been verified by an admin
    pub verified: bool,
} 