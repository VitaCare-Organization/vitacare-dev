use soroban_sdk::{contracttype, Error as SorobanError};

#[contracttype]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    HostError = 0,
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    AppointmentNotFound = 4,
    InvalidStatus = 5,
    PastTimestamp = 6,
}

impl From<Error> for SorobanError {
    fn from(err: Error) -> SorobanError {
        SorobanError::from_contract_error(err as u32)
    }
}

impl From<&Error> for SorobanError {
    fn from(err: &Error) -> SorobanError {
        SorobanError::from_contract_error(*err as u32)
    }
}

impl From<SorobanError> for Error {
    fn from(_err: SorobanError) -> Error {
        Error::HostError
    }
}