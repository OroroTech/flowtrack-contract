use soroban_sdk::{contracttype, Address, Env};

use crate::errors::ContractError;
use crate::types::{DisbursementSchedule, Recipient};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Recipient(Address),
    Schedule(Address),
}

pub fn save_recipient(env: &Env, recipient: &Recipient) {
    let key = DataKey::Recipient(recipient.address.clone());
    env.storage().persistent().set(&key, recipient);
}

pub fn get_recipient(env: &Env, address: &Address) -> Result<Recipient, ContractError> {
    let key = DataKey::Recipient(address.clone());
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(ContractError::RecipientNotFound)
}

pub fn has_recipient(env: &Env, address: &Address) -> bool {
    let key = DataKey::Recipient(address.clone());
    env.storage().persistent().has(&key)
}

pub fn save_schedule(env: &Env, schedule: &DisbursementSchedule) {
    let key = DataKey::Schedule(schedule.recipient.clone());
    env.storage().persistent().set(&key, schedule);
}

pub fn get_schedule(env: &Env, address: &Address) -> Result<DisbursementSchedule, ContractError> {
    let key = DataKey::Schedule(address.clone());
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(ContractError::ScheduleNotFound)
}
