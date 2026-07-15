# flowtrack-contract

Soroban smart contract for the Stellar Disbursement Tracker — an open-source
transparency tool for tracking on-chain disbursements on Stellar, built for
recipients, auditors, and the public.

## What this contract does

The contract handles recipient enrollment and disbursement scheduling on-chain,
following SEP-41 interface patterns for structured data. It is the source of
truth that the tracker's backend listens to and aggregates payment history
from.

Functions:

| Function | Params | Returns | Description |
|---|---|---|---|
| `enroll_recipient` | `recipient: Recipient` | `Result<(), ContractError>` | Enrolls a new recipient. Fails if the address is already enrolled or the name is empty. Emits a `RecipientEnrolled` event. |
| `schedule_disbursement` | `schedule: DisbursementSchedule` | `Result<(), ContractError>` | Schedules a recurring disbursement for an enrolled recipient. Fails if the recipient isn't enrolled or the amount isn't positive. Emits a `DisbursementScheduled` event. |
| `get_recipient` | `address: Address` | `Result<Recipient, ContractError>` | Looks up a recipient's enrollment record. |
| `get_schedule` | `address: Address` | `Result<DisbursementSchedule, ContractError>` | Looks up a recipient's disbursement schedule. |

### Types

- `Recipient { address: Address, name: String, enrolled_at: u64 }`
- `DisbursementSchedule { recipient: Address, amount: i128, currency: Symbol, interval_seconds: u64, next_disbursement_at: u64 }`

### Error codes

| Error | Meaning |
|---|---|
| `AlreadyEnrolled` | `enroll_recipient` called for an address that's already enrolled. |
| `RecipientNotFound` | Lookup or scheduling referenced an address with no enrollment record. |
| `InvalidAmount` | Enrollment had an empty name, or a schedule's amount was not greater than zero. |
| `Unauthorized` | Reserved for actions gated to a specific caller. |
| `ScheduleNotFound` | `get_schedule` called for an address with no disbursement schedule. |

## Storage strategy

Recipient and schedule records use **persistent storage**
(`env.storage().persistent()`), keyed by a `DataKey` enum (`Recipient(Address)`
/ `Schedule(Address)`). Persistent storage is required here because this data
must survive ledger expiry (TTL) bumps — recipients may go months between
disbursements, and their enrollment record must still be readable when the
next payment is due. Instance storage is reserved for contract-wide
config/admin state, which this contract doesn't yet have.

## Building

```bash
cargo build --target wasm32v1-none --release
```

`wasm32v1-none` is the Soroban-supported wasm target for current Rust
toolchains (1.84+) — the older `wasm32-unknown-unknown` target enables
features Soroban's environment doesn't yet support.

## Testing

```bash
cargo test
```

## Deploying

Requires the [Stellar CLI](https://developers.stellar.org/docs/tools/cli/stellar-cli):

```bash
cargo install --locked stellar-cli

stellar contract deploy \
  --wasm target/wasm32v1-none/release/stellar_disbursement_tracker_contract.wasm \
  --source <your-identity> \
  --network testnet
```
