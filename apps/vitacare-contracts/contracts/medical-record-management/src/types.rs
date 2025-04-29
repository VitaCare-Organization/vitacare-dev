use soroban_sdk::{Address, BytesN, String, contracttype};

/// Simple counter to generate sequential IDs
#[contracttype]
#[derive(Clone)]
pub struct Counter(pub u32);

/// Type to represent a medical record identifier
pub type RecordId = u32;

/// Complete medical record
#[contracttype]
#[derive(Clone)]
pub struct MedicalRecord {
    pub id: RecordId,
    pub patient: Address,
    pub doctor: Address,
    pub timestamp: u64,
    pub notes: String,
    pub diagnosis: String,
    pub treatment: String,
}

/// Structure for storing medical record metadata
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordMetadata {
    /// Unique record ID
    pub id: RecordId,
    /// Patient who owns the record
    pub patient: Address,
    /// Doctor who created the record
    pub doctor: Address,
    /// Identifier or hash of the medical data
    pub data_id: BytesN<32>,
    /// Short description of the record
    pub description: String,
} 