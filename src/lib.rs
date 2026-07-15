#![no_std]

mod errors;
mod storage;
#[cfg(test)]
mod test;
mod types;

use soroban_sdk::{contract, contractimpl, Address, Env};

pub use errors::ContractError;
pub use types::{DisbursementSchedule, DisbursementScheduled, Recipient, RecipientEnrolled};

#[contract]
pub struct DisbursementTrackerContract;

#[contractimpl]
impl DisbursementTrackerContract {
    pub fn enroll_recipient(env: Env, recipient: Recipient) -> Result<(), ContractError> {
        if storage::has_recipient(&env, &recipient.address) {
            return Err(ContractError::AlreadyEnrolled);
        }
        if recipient.name.is_empty() {
            return Err(ContractError::InvalidAmount);
        }

        storage::save_recipient(&env, &recipient);

        RecipientEnrolled {
            recipient: recipient.address.clone(),
            enrolled_at: recipient.enrolled_at,
        }
        .publish(&env);

        Ok(())
    }

    pub fn schedule_disbursement(
        env: Env,
        schedule: DisbursementSchedule,
    ) -> Result<(), ContractError> {
        storage::get_recipient(&env, &schedule.recipient)?;

        if schedule.amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        storage::save_schedule(&env, &schedule);

        DisbursementScheduled {
            recipient: schedule.recipient.clone(),
            amount: schedule.amount,
        }
        .publish(&env);

        Ok(())
    }

    pub fn get_recipient(env: Env, address: Address) -> Result<Recipient, ContractError> {
        storage::get_recipient(&env, &address)
    }

    pub fn get_schedule(env: Env, address: Address) -> Result<DisbursementSchedule, ContractError> {
        storage::get_schedule(&env, &address)
    }
}
