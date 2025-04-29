use soroban_sdk::{contracterror, contracttype, Address, Env, Map, String, Vec};

/// Represents a coverage policy that the insurer offers
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoveragePolicy {
    /// Unique identifier for the policy
    pub id: String,
    /// Name of the policy
    pub name: String,
    /// Description of what the policy covers
    pub description: String,
    /// Premium amount in smallest currency unit
    pub premium_amount: u64,
    /// Coverage amount in smallest currency unit
    pub coverage_amount: u64,
    /// Duration of coverage in days
    pub duration_days: u32,
}

/// Represents an authorized claims reviewer for an insurance company
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaimsReviewer {
    /// Wallet address of the reviewer
    pub wallet: Address,
    /// Name of the reviewer
    pub name: String,
    /// Role of the reviewer
    pub role: String,
    /// Whether the reviewer is active
    pub active: bool,
}

/// Represents an insurance company
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsurerData {
    /// Wallet address of the insurer
    pub wallet: Address,
    /// Name of the insurance company
    pub name: String,
    /// Government-issued license ID
    pub license_id: String,
    /// Additional metadata (JSON string)
    pub metadata: String,
    /// Available coverage policies
    pub coverage_policies: Vec<CoveragePolicy>,
    /// Registered claims reviewers
    pub claims_reviewers: Vec<ClaimsReviewer>,
    /// Whether the insurer is active
    pub active: bool,
    /// Registration timestamp
    pub registered_at: u64,
    /// Last updated timestamp
    pub updated_at: u64,
}

/// Enum representing all data keys used for storage in the contract
#[derive(Clone)]
pub enum DataKey {
    /// Map from wallet addresses to unique IDs
    InsurerMap,
    /// List of all registered insurers
    AllInsurers,
    /// Individual insurer data
    Insurer(Address),
}

impl DataKey {
    /// Convert the key to a Soroban String
    pub fn to_string(&self, env: &Env) -> String {
        match self {
            DataKey::InsurerMap => String::from_str(env, "insurer_map"),
            DataKey::AllInsurers => String::from_str(env, "all_insurers"),
            DataKey::Insurer(wallet) => {
                // Use the same logic from get_insurer_key but encapsulated in the enum
                // Create a mapping from wallet addresses to unique IDs
                let map_key = DataKey::InsurerMap.to_string(env);

                // Check if we already have an ID for this wallet
                let wallet_map: Map<Address, u64> = env
                    .storage()
                    .instance()
                    .get(&map_key)
                    .unwrap_or_else(|| Map::new(env));

                // Get or create an ID for this wallet
                let wallet_id = if let Some(id) = wallet_map.get(wallet.clone()) {
                    id
                } else {
                    // Create a new ID for this wallet
                    let new_id = wallet_map.len() as u64;
                    let mut new_map = wallet_map.clone();
                    new_map.set(wallet.clone(), new_id);
                    env.storage().instance().set(&map_key, &new_map);
                    new_id
                };

                // Create a key with this wallet's unique ID
                String::from_str(env, &format!("insurer_data_wallet_{}", wallet_id))
            }
        }
    }
}

/// Defines errors that can be returned by the contract
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum InsurerError {
    /// When an insurer doesn't exist
    InsurerNotFound = 1,
    /// When an insurer already exists
    InsurerAlreadyExists = 2,
    /// When the caller is not authorized
    Unauthorized = 3,
    /// When input data is invalid
    InvalidInput = 4,
}
