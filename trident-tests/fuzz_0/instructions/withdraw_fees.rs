use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("B6VvVi2nqC5vQfDUxEjcBNCHN7r1kBMZGSAAJyUg3e2j")]
#[discriminator([198u8, 212u8, 171u8, 109u8, 144u8, 215u8, 174u8, 89u8])]
pub struct WithdrawFeesInstruction {
    pub accounts: WithdrawFeesInstructionAccounts,
    pub data: WithdrawFeesInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(WithdrawFeesInstructionData)]
#[storage(FuzzAccounts)]
pub struct WithdrawFeesInstructionAccounts {
    #[account(
        mut,
        signer,
        storage::name = authority,
        storage::account_id = (0..1)
    )]
    pub authority: TridentAccount,

    #[account(
        mut,
        storage::name = global_config,
        storage::account_id = (0..1),
        seeds = [b"global_config"],
    )]
    pub global_config: TridentAccount,

    #[account(
        mut,
        storage::name = treasury,
        storage::account_id = (0..1),
        seeds = [b"treasury"],
    )]
    pub treasury: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct WithdrawFeesInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for WithdrawFeesInstruction {
    type IxAccounts = FuzzAccounts;
}
