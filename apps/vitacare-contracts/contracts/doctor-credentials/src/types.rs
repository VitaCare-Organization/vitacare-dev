use soroban_sdk::{
    contracterror, contracttype, Address, Bytes, Env, String, TryFromVal, Val,
};

/// Represents the data structure for a doctor's credentials
#[contracttype]
#[derive(Clone)]
pub struct DoctorData {
    /// The doctor's full name
    pub name: String,
    /// The doctor's medical specialization
    pub specialization: String,
    /// Hash of the doctor's medical certificate
    pub certificate_hash: Bytes,
    /// Whether the doctor has been verified by an institution
    pub is_verified: bool,
    /// The address of the institution that verified the doctor
    pub verified_by: Option<Address>,
}

/// Storage keys for the contract
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Key for storing doctor data
    Doctor(Address),
    /// Key for storing institution verification status
    Institution(Address),
}

/// Error types that can occur in the contract
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    /// Doctor is already registered
    DoctorAlreadyExists = 1,
    /// Doctor is not found
    DoctorNotFound = 2,
    /// Institution is not verified
    InstitutionNotVerified = 3,
    /// Invalid input data
    InvalidInput = 4,
}

impl From<ContractError> for String {
    fn from(error: ContractError) -> Self {
        match error {
            ContractError::DoctorAlreadyExists => String::from_str(&soroban_sdk::Env::default(), "Doctor already registered"),
            ContractError::DoctorNotFound => String::from_str(&soroban_sdk::Env::default(), "Doctor not found"),
            ContractError::InstitutionNotVerified => String::from_str(&soroban_sdk::Env::default(), "Institution not verified"),
            ContractError::InvalidInput => String::from_str(&soroban_sdk::Env::default(), "Invalid input data"),
        }
    }
} 