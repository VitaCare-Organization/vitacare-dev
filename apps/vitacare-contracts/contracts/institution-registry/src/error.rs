use soroban_sdk::contracterror;

/// Custom error codes for the Institution Registry contract
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Error when attempting to register an institution that already exists
    InstitutionAlreadyRegistered = 1,
    
    /// Error when attempting to access or update a non-existent institution
    InstitutionNotFound = 2,
    
    /// Error when an operation requires admin privileges but is called by a non-admin
    Unauthorized = 3,
} 