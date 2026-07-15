use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyEnrolled = 1,
    RecipientNotFound = 2,
    InvalidAmount = 3,
    Unauthorized = 4,
    ScheduleNotFound = 5,
}
