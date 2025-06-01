use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum DoctorError {
    DoctorNotFound = 1,
    DoctorAlreadyExists = 2,
    InvalidInput = 3,
    Unauthorized = 4,
} 