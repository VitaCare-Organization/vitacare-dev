#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, 
    Vec, symbol_short, contracterror, BytesN,
    xdr::ToXdr,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    Unauthorized = 2,
    ClaimNotFound = 3,
    InvalidAmount = 4,
    ClaimAlreadyProcessed = 5,
}

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
    patient: Address,
    service_id: u64,
    cost: i128,
    status: ClaimStatus,
    insurer: Option<Address>,
}

#[contract]
pub struct InsuranceClaims;

#[contractimpl]
impl InsuranceClaims {
    // Initialize contract with verified insurers
    pub fn initialize(env: Env, admin: Address, insurers: Vec<Address>) -> Result<(), Error> {
        admin.require_auth();
        
        let admin_key = symbol_short!("ADMIN");
        env.storage().instance().set(&admin_key, &admin);
        
        let insurers_key = symbol_short!("INSURERS");
        env.storage().instance().set(&insurers_key, &insurers);
        
        let next_claim_id_key = symbol_short!("NEXT_ID");
        env.storage().instance().set(&next_claim_id_key, &0u64);
        
        Ok(())
    }
    
    // Add a new insurer (admin only)
    pub fn add_insurer(env: Env, admin: Address, insurer: Address) -> Result<(), Error> {
        // Get admin and require authorization
        let stored_admin: Address = env.storage().instance().get(&symbol_short!("ADMIN")).ok_or(Error::NotInitialized)?;
        
        // Verify the admin is the one stored
        if admin != stored_admin {
            return Err(Error::Unauthorized);
        }
        
        admin.require_auth();
        
        // Get current insurers list
        let mut insurers: Vec<Address> = env.storage().instance().get(&symbol_short!("INSURERS")).ok_or(Error::NotInitialized)?;
        
        // Only add if not already in list
        if !insurers.contains(&insurer) {
            insurers.push_back(insurer);
            env.storage().instance().set(&symbol_short!("INSURERS"), &insurers);
        }
        
        Ok(())
    }
    
    // Submit a new insurance claim
    pub fn submit_claim(env: Env, patient: Address, service_id: u64, cost: i128) -> Result<u64, Error> {
        patient.require_auth();
        
        if cost <= 0 {
            return Err(Error::InvalidAmount);
        }
        
        // Get next claim ID and increment
        let next_id_key = symbol_short!("NEXT_ID");
        let claim_id: u64 = env.storage().instance().get(&next_id_key).ok_or(Error::NotInitialized)?;
        env.storage().instance().set(&next_id_key, &(claim_id + 1));
        
        // Create new claim with pending status
        let claim = Claim {
            patient: patient.clone(),
            service_id,
            cost,
            status: ClaimStatus::Pending,
            insurer: None,
        };
        
        // Store claim data
        env.storage().persistent().set(&claim_id, &claim);
        
        // Add claim to patient's claims list using a hash of the patient address
        let patient_key = get_patient_key(&env, &patient);
        let mut patient_claims: Vec<u64> = env.storage().persistent().get(&patient_key).unwrap_or(Vec::new(&env));
        patient_claims.push_back(claim_id);
        env.storage().persistent().set(&patient_key, &patient_claims);
        
        Ok(claim_id)
    }
    
    // Process claim (approve or reject)
    pub fn process_claim(env: Env, insurer: Address, claim_id: u64, approve: bool) -> Result<(), Error> {
        insurer.require_auth();
        
        // Check if insurer is verified
        let insurers: Vec<Address> = env.storage().instance().get(&symbol_short!("INSURERS")).ok_or(Error::NotInitialized)?;
        if !insurers.contains(&insurer) {
            return Err(Error::Unauthorized);
        }
        
        // Get claim data
        let mut claim: Claim = env.storage().persistent().get(&claim_id).ok_or(Error::ClaimNotFound)?;
        
        // Check if claim is still pending
        if claim.status != ClaimStatus::Pending {
            return Err(Error::ClaimAlreadyProcessed);
        }
        
        // Update claim status
        claim.status = if approve { ClaimStatus::Approved } else { ClaimStatus::Rejected };
        claim.insurer = Some(insurer);
        
        // Save updated claim
        env.storage().persistent().set(&claim_id, &claim);
        
        Ok(())
    }
    
    // Get claim status
    pub fn get_claim_status(env: Env, claim_id: u64) -> Result<ClaimStatus, Error> {
        let claim: Claim = env.storage().persistent().get(&claim_id).ok_or(Error::ClaimNotFound)?;
        Ok(claim.status)
    }
    
    // Get full claim details
    pub fn get_claim_details(env: Env, claim_id: u64) -> Result<Claim, Error> {
        let claim: Claim = env.storage().persistent().get(&claim_id).ok_or(Error::ClaimNotFound)?;
        Ok(claim)
    }
    
    // Get all claims for a patient
    pub fn get_patient_claims(env: Env, patient: Address) -> Vec<u64> {
        let patient_key = get_patient_key(&env, &patient);
        env.storage().persistent().get(&patient_key).unwrap_or(Vec::new(&env))
    }
}

// Helper function to get a consistent key for patient claims
fn get_patient_key(env: &Env, patient: &Address) -> BytesN<32> {
    let xdr_bytes = patient.to_xdr(env);
    env.crypto().sha256(&xdr_bytes).into()
}

mod test; 