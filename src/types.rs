use soroban_sdk::{contracttype, Address, String, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Recipient {
    pub address: Address,
    pub name: String,
    pub enrolled_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisbursementSchedule {
    pub recipient: Address,
    pub amount: i128,
    pub currency: Symbol,
    pub interval_seconds: u64,
    pub next_disbursement_at: u64,
}
