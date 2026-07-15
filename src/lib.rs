#![no_std]

mod errors;
mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, symbol_short, Env};

pub use errors::ContractError;
pub use types::{DisbursementSchedule, Recipient};

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

        env.events().publish(
            (symbol_short!("enrolled"), recipient.address.clone()),
            recipient.enrolled_at,
        );

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

        env.events().publish(
            (symbol_short!("scheduled"), schedule.recipient.clone()),
            schedule.amount,
        );

        Ok(())
    }
}
