use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("B6VvVi2nqC5vQfDUxEjcBNCHN7r1kBMZGSAAJyUg3e2j")]
#[discriminator([119u8, 66u8, 44u8, 236u8, 79u8, 158u8, 82u8, 51u8])]
pub struct SelectWinnerInstruction {
    pub accounts: SelectWinnerInstructionAccounts,
    pub data: SelectWinnerInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(SelectWinnerInstructionData)]
#[storage(FuzzAccounts)]
pub struct SelectWinnerInstructionAccounts {
    #[account(mut, signer)]
    pub signer: TridentAccount,

    pub global_state: TridentAccount,

    #[account(mut)]
    pub lottery: TridentAccount,

    pub randomness_account: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SelectWinnerInstructionData {
    pub lottery_id: u64,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for SelectWinnerInstruction {
    type IxAccounts = FuzzAccounts;
}
