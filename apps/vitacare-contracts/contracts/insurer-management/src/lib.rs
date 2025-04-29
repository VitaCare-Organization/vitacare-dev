use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

mod insurer;
mod test;
mod types;

use crate::insurer::{validate_string_not_empty, InsurerOperations};
use crate::types::{ClaimsReviewer, CoveragePolicy, InsurerData, InsurerError};

/// InsurerManagement contract trait definition
pub trait InsurerManagementTrait {
    /// Register a new insurer with basic information
    fn register_insurer(
        env: Env,
        wallet: Address,
        name: String,
        license_id: String,
        metadata: String,
    ) -> Result<(), InsurerError>;

    /// Update insurer's metadata (must be called by the insurer)
    fn update_insurer(env: Env, wallet: Address, metadata: String) -> Result<(), InsurerError>;

    /// Get insurer data by wallet address
    fn get_insurer(env: Env, wallet: Address) -> Result<InsurerData, InsurerError>;

    /// Add a coverage policy to an insurer (must be called by the insurer)
    fn add_coverage_policy(
        env: Env,
        wallet: Address,
        id: String,
        name: String,
        description: String,
        premium_amount: u64,
        coverage_amount: u64,
        duration_days: u32,
    ) -> Result<(), InsurerError>;

    /// Update an existing coverage policy (must be called by the insurer)
    fn update_coverage_policy(
        env: Env,
        wallet: Address,
        id: String,
        name: String,
        description: String,
        premium_amount: u64,
        coverage_amount: u64,
        duration_days: u32,
    ) -> Result<(), InsurerError>;

    /// Add a claims reviewer to an insurer (must be called by the insurer)
    fn add_claims_reviewer(
        env: Env,
        insurer_wallet: Address,
        reviewer_wallet: Address,
        name: String,
        role: String,
    ) -> Result<(), InsurerError>;

    /// Update a claims reviewer's details (must be called by the insurer)
    fn update_claims_reviewer(
        env: Env,
        insurer_wallet: Address,
        reviewer_wallet: Address,
        name: String,
        role: String,
        active: bool,
    ) -> Result<(), InsurerError>;

    /// Deactivate an insurer (must be called by the insurer)
    fn deactivate_insurer(env: Env, wallet: Address) -> Result<(), InsurerError>;

    /// Reactivate an insurer (must be called by the insurer)
    fn reactivate_insurer(env: Env, wallet: Address) -> Result<(), InsurerError>;

    /// Get all registered insurers
    fn get_all_insurers(env: Env) -> Vec<Address>;
}

#[contract]
pub struct InsurerManagement;

#[contractimpl]
impl InsurerManagementTrait for InsurerManagement {
    fn register_insurer(
        env: Env,
        wallet: Address,
        name: String,
        license_id: String,
        metadata: String,
    ) -> Result<(), InsurerError> {
        // Validate inputs
        validate_string_not_empty(&name)?;
        validate_string_not_empty(&license_id)?;
        validate_string_not_empty(&metadata)?;

        // Check if insurer already exists
        if InsurerOperations::insurer_exists(&env, &wallet) {
            return Err(InsurerError::InsurerAlreadyExists);
        }

        // Create new insurer data
        let insurer = InsurerData {
            wallet: wallet.clone(),
            name,
            license_id,
            metadata,
            coverage_policies: Vec::new(&env),
            claims_reviewers: Vec::new(&env),
            active: true,
            registered_at: env.ledger().timestamp(),
            updated_at: env.ledger().timestamp(),
        };

        // Save insurer data
        InsurerOperations::save_insurer(&env, &insurer);

        Ok(())
    }

    fn update_insurer(env: Env, wallet: Address, metadata: String) -> Result<(), InsurerError> {
        // Verify caller is the insurer
        wallet.require_auth();

        // Get existing insurer data
        let mut insurer = match InsurerOperations::get_insurer(&env, &wallet) {
            Some(data) => data,
            None => return Err(InsurerError::InsurerNotFound),
        };

        // Update metadata and timestamp
        insurer.metadata = metadata;
        insurer.updated_at = env.ledger().timestamp();

        // Save updated insurer data
        InsurerOperations::save_insurer(&env, &insurer);

        Ok(())
    }

    fn get_insurer(env: Env, wallet: Address) -> Result<InsurerData, InsurerError> {
        // Get insurer data
        match InsurerOperations::get_insurer(&env, &wallet) {
            Some(data) => Ok(data),
            None => Err(InsurerError::InsurerNotFound),
        }
    }

    fn add_coverage_policy(
        env: Env,
        wallet: Address,
        id: String,
        name: String,
        description: String,
        premium_amount: u64,
        coverage_amount: u64,
        duration_days: u32,
    ) -> Result<(), InsurerError> {
        // Verify caller is the insurer
        wallet.require_auth();

        // Validate inputs
        validate_string_not_empty(&id)?;
        validate_string_not_empty(&name)?;

        // Create policy
        let policy = CoveragePolicy {
            id,
            name,
            description,
            premium_amount,
            coverage_amount,
            duration_days,
        };

        // Add policy to insurer
        let result = InsurerOperations::add_coverage_policy(&env, &wallet, &policy);

        // If insurer not found, return error; otherwise success
        // (even if policy already exists, we treat it as success)
        if result == false {
            return Err(InsurerError::InsurerNotFound);
        }

        Ok(())
    }

    fn update_coverage_policy(
        env: Env,
        wallet: Address,
        id: String,
        name: String,
        description: String,
        premium_amount: u64,
        coverage_amount: u64,
        duration_days: u32,
    ) -> Result<(), InsurerError> {
        // Verify caller is the insurer
        wallet.require_auth();

        // Validate inputs
        validate_string_not_empty(&id)?;
        validate_string_not_empty(&name)?;

        // Create updated policy
        let policy = CoveragePolicy {
            id,
            name,
            description,
            premium_amount,
            coverage_amount,
            duration_days,
        };

        // Update policy
        if !InsurerOperations::update_coverage_policy(&env, &wallet, &policy) {
            return Err(InsurerError::InsurerNotFound);
        }

        Ok(())
    }

    fn add_claims_reviewer(
        env: Env,
        insurer_wallet: Address,
        reviewer_wallet: Address,
        name: String,
        role: String,
    ) -> Result<(), InsurerError> {
        // Verify caller is the insurer
        insurer_wallet.require_auth();

        // Validate inputs
        validate_string_not_empty(&name)?;
        validate_string_not_empty(&role)?;

        // Create reviewer
        let reviewer = ClaimsReviewer {
            wallet: reviewer_wallet,
            name,
            role,
            active: true,
        };

        // Add reviewer to insurer
        let result = InsurerOperations::add_claims_reviewer(&env, &insurer_wallet, &reviewer);

        // If insurer not found, return error; otherwise success
        // (even if reviewer already exists, we treat it as success)
        if result == false {
            return Err(InsurerError::InsurerNotFound);
        }

        Ok(())
    }

    fn update_claims_reviewer(
        env: Env,
        insurer_wallet: Address,
        reviewer_wallet: Address,
        name: String,
        role: String,
        active: bool,
    ) -> Result<(), InsurerError> {
        // Verify caller is the insurer
        insurer_wallet.require_auth();

        // Validate inputs
        validate_string_not_empty(&name)?;
        validate_string_not_empty(&role)?;

        // Create updated reviewer
        let reviewer = ClaimsReviewer {
            wallet: reviewer_wallet,
            name,
            role,
            active,
        };

        // Update reviewer
        if !InsurerOperations::update_claims_reviewer(&env, &insurer_wallet, &reviewer) {
            return Err(InsurerError::InsurerNotFound);
        }

        Ok(())
    }

    fn deactivate_insurer(env: Env, wallet: Address) -> Result<(), InsurerError> {
        // Verify caller is the insurer
        wallet.require_auth();

        // Get existing insurer data
        let mut insurer = match InsurerOperations::get_insurer(&env, &wallet) {
            Some(data) => data,
            None => return Err(InsurerError::InsurerNotFound),
        };

        // Set active to false and update timestamp
        insurer.active = false;
        insurer.updated_at = env.ledger().timestamp();

        // Save updated insurer data
        InsurerOperations::save_insurer(&env, &insurer);

        Ok(())
    }

    fn reactivate_insurer(env: Env, wallet: Address) -> Result<(), InsurerError> {
        // Verify caller is the insurer
        wallet.require_auth();

        // Get existing insurer data
        let mut insurer = match InsurerOperations::get_insurer(&env, &wallet) {
            Some(data) => data,
            None => return Err(InsurerError::InsurerNotFound),
        };

        // Set active to true and update timestamp
        insurer.active = true;
        insurer.updated_at = env.ledger().timestamp();

        // Save updated insurer data
        InsurerOperations::save_insurer(&env, &insurer);

        Ok(())
    }

    fn get_all_insurers(env: Env) -> Vec<Address> {
        InsurerOperations::get_all_insurers(&env)
    }
}
