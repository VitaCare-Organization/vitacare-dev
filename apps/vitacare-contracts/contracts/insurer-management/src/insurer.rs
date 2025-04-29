use crate::types::{ClaimsReviewer, CoveragePolicy, DataKey, InsurerData, InsurerError};
use soroban_sdk::{Address, Env, String, Vec};

/// Storage functions for managing insurer data
pub struct InsurerOperations;

impl InsurerOperations {
    /// Store insurer data
    pub fn save_insurer(env: &Env, insurer: &InsurerData) {
        let key = DataKey::Insurer(insurer.wallet.clone()).to_string(env);
        env.storage().instance().set(&key, insurer);

        // Update the list of all insurers
        Self::add_to_all_insurers(env, &insurer.wallet);
    }

    /// Get insurer data by wallet address
    pub fn get_insurer(env: &Env, wallet: &Address) -> Option<InsurerData> {
        let key = DataKey::Insurer(wallet.clone()).to_string(env);
        env.storage().instance().get(&key)
    }

    /// Check if an insurer exists
    pub fn insurer_exists(env: &Env, wallet: &Address) -> bool {
        let key = DataKey::Insurer(wallet.clone()).to_string(env);
        env.storage().instance().has(&key)
    }

    /// Add a wallet address to the list of all insurers
    fn add_to_all_insurers(env: &Env, wallet: &Address) {
        let key = DataKey::AllInsurers.to_string(env);
        let mut all_insurers: Vec<Address> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| Vec::new(env));

        // Only add if not already in the list
        if !all_insurers.contains(wallet) {
            all_insurers.push_back(wallet.clone());
            env.storage().instance().set(&key, &all_insurers);
        }
    }

    /// Get all insurer wallet addresses
    pub fn get_all_insurers(env: &Env) -> Vec<Address> {
        let key = DataKey::AllInsurers.to_string(env);
        env.storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| Vec::new(env))
    }

    /// Add a coverage policy to an insurer
    pub fn add_coverage_policy(env: &Env, wallet: &Address, policy: &CoveragePolicy) -> bool {
        if let Some(mut insurer) = Self::get_insurer(env, wallet) {
            // Check if policy with same ID already exists
            for existing in insurer.coverage_policies.iter() {
                if existing.id == policy.id {
                    // Policy already exists, so return true (success)
                    return true;
                }
            }

            insurer.coverage_policies.push_back(policy.clone());
            insurer.updated_at = env.ledger().timestamp();
            Self::save_insurer(env, &insurer);
            return true;
        }
        false
    }

    /// Update an existing coverage policy
    pub fn update_coverage_policy(env: &Env, wallet: &Address, policy: &CoveragePolicy) -> bool {
        if let Some(mut insurer) = Self::get_insurer(env, wallet) {
            let mut found = false;

            for i in 0..insurer.coverage_policies.len() {
                if insurer.coverage_policies.get(i).unwrap().id == policy.id {
                    insurer.coverage_policies.set(i, policy.clone());
                    found = true;
                    break;
                }
            }

            if found {
                insurer.updated_at = env.ledger().timestamp();
                Self::save_insurer(env, &insurer);
                return true;
            }
        }
        false
    }

    /// Add a claims reviewer to an insurer
    pub fn add_claims_reviewer(env: &Env, wallet: &Address, reviewer: &ClaimsReviewer) -> bool {
        if let Some(mut insurer) = Self::get_insurer(env, wallet) {
            // Check if reviewer with same wallet already exists
            for existing in insurer.claims_reviewers.iter() {
                if existing.wallet == reviewer.wallet {
                    // Reviewer already exists, so return true (success)
                    return true;
                }
            }

            insurer.claims_reviewers.push_back(reviewer.clone());
            insurer.updated_at = env.ledger().timestamp();
            Self::save_insurer(env, &insurer);
            return true;
        }
        false
    }

    /// Update an existing claims reviewer
    pub fn update_claims_reviewer(env: &Env, wallet: &Address, reviewer: &ClaimsReviewer) -> bool {
        if let Some(mut insurer) = Self::get_insurer(env, wallet) {
            let mut found = false;

            for i in 0..insurer.claims_reviewers.len() {
                if insurer.claims_reviewers.get(i).unwrap().wallet == reviewer.wallet {
                    insurer.claims_reviewers.set(i, reviewer.clone());
                    found = true;
                    break;
                }
            }

            if found {
                insurer.updated_at = env.ledger().timestamp();
                Self::save_insurer(env, &insurer);
                return true;
            }
        }
        false
    }
}

/// Helper function to check if an input string 4is valid (not empty)
pub fn validate_string_not_empty(s: &String) -> Result<(), InsurerError> {
    if s.is_empty() {
        return Err(InsurerError::InvalidInput);
    }
    Ok(())
}
