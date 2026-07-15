#![cfg(test)]

use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env, String};

use crate::{
    DisbursementSchedule, DisbursementTrackerContract, DisbursementTrackerContractClient,
    Recipient,
};

fn setup(env: &Env) -> DisbursementTrackerContractClient<'_> {
    let contract_id = env.register(DisbursementTrackerContract, ());
    DisbursementTrackerContractClient::new(env, &contract_id)
}

#[test]
fn test_enroll_recipient_success() {
    let env = Env::default();
    let client = setup(&env);
    let address = Address::generate(&env);

    let recipient = Recipient {
        address: address.clone(),
        name: String::from_str(&env, "Alice"),
        enrolled_at: 1_000,
    };

    client.enroll_recipient(&recipient);

    let stored = client.get_recipient(&address);
    assert_eq!(stored, recipient);
}

#[test]
fn test_enroll_duplicate_fails() {
    let env = Env::default();
    let client = setup(&env);
    let address = Address::generate(&env);

    let recipient = Recipient {
        address: address.clone(),
        name: String::from_str(&env, "Alice"),
        enrolled_at: 1_000,
    };

    client.enroll_recipient(&recipient);

    let result = client.try_enroll_recipient(&recipient);
    assert_eq!(
        result,
        Err(Ok(crate::ContractError::AlreadyEnrolled))
    );
}

#[test]
fn test_schedule_disbursement_success() {
    let env = Env::default();
    let client = setup(&env);
    let address = Address::generate(&env);

    let recipient = Recipient {
        address: address.clone(),
        name: String::from_str(&env, "Alice"),
        enrolled_at: 1_000,
    };
    client.enroll_recipient(&recipient);

    let schedule = DisbursementSchedule {
        recipient: address.clone(),
        amount: 500,
        currency: symbol_short!("USDC"),
        interval_seconds: 86_400,
        next_disbursement_at: 2_000,
    };
    client.schedule_disbursement(&schedule);

    let stored = client.get_schedule(&address);
    assert_eq!(stored, schedule);
}

#[test]
fn test_schedule_without_enrollment_fails() {
    let env = Env::default();
    let client = setup(&env);
    let address = Address::generate(&env);

    let schedule = DisbursementSchedule {
        recipient: address.clone(),
        amount: 500,
        currency: symbol_short!("USDC"),
        interval_seconds: 86_400,
        next_disbursement_at: 2_000,
    };

    let result = client.try_schedule_disbursement(&schedule);
    assert_eq!(
        result,
        Err(Ok(crate::ContractError::RecipientNotFound))
    );
}

#[test]
fn test_get_recipient_not_found() {
    let env = Env::default();
    let client = setup(&env);
    let address = Address::generate(&env);

    let result = client.try_get_recipient(&address);
    assert_eq!(
        result,
        Err(Ok(crate::ContractError::RecipientNotFound))
    );
}
