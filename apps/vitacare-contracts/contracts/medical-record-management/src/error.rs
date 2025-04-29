use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    // Access control errors
    Unauthorized = 1,
    AccessAlreadyGranted = 2,
    AccessNotGranted = 3,
    
    // Record management errors
    RecordNotFound = 4,
    InvalidRecordData = 5,
    
    // General errors
    InternalError = 100,
} 