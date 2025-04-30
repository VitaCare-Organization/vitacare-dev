use soroban_sdk::{
    contracttype, Address, Env, symbol_short, BytesN,
    Vec, xdr::ToXdr,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClaimStatus {
    Pending,
    Approved,
    Rejected,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Claim {
    pub patient: Address,
    pub service_id: u64,
    pub cost: i128,
    pub status: ClaimStatus,
    pub insurer: Option<Address>,
}

// Helper function to get a consistent key for patient claims
pub fn get_patient_key(env: &Env, patient: &Address) -> BytesN<32> {
    let xdr_bytes = patient.to_xdr(env);
    env.crypto().sha256(&xdr_bytes).into()
}

// Storage keys
pub fn admin_key() -> &'static [u8] {
    symbol_short!("ADMIN")
}

pub fn insurers_key() -> &'static [u8] {
    symbol_short!("INSURERS")
}

pub fn next_claim_id_key() -> &'static [u8] {
    symbol_short!("NEXT_ID")
}