use soroban_sdk::{contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DoctorProfile {
    pub wallet: Address,
    pub name: String,
    pub specialization: String,
    pub institution_wallet: Address,
    pub metadata: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl DoctorProfile {
    pub fn new(
        env: &Env,
        wallet: Address,
        name: String,
        specialization: String,
        institution_wallet: Address,
    ) -> Self {
        Self {
            wallet,
            name,
            specialization,
            institution_wallet,
            metadata: String::from_str(env, ""),
            created_at: env.ledger().timestamp(),
            updated_at: env.ledger().timestamp(),
        }
    }
} 