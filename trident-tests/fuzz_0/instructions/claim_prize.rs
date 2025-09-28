use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("B6VvVi2nqC5vQfDUxEjcBNCHN7r1kBMZGSAAJyUg3e2j")]
#[discriminator([157u8, 233u8, 139u8, 121u8, 246u8, 62u8, 234u8, 235u8])]
pub struct ClaimPrizeInstruction {
    pub accounts: ClaimPrizeInstructionAccounts,
    pub data: ClaimPrizeInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(ClaimPrizeInstructionData)]
#[storage(FuzzAccounts)]
pub struct ClaimPrizeInstructionAccounts {
    #[account(mut, signer)]
    pub claimer: TridentAccount,

    #[account(mut)]
    pub global_config: TridentAccount,

    #[account(mut)]
    pub treasury: TridentAccount,

    pub lottery: TridentAccount,

    pub ticket_mint: TridentAccount,

    pub claimer_token_account: TridentAccount,

    pub token_program: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct ClaimPrizeInstructionData {
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
impl InstructionHooks for ClaimPrizeInstruction {
    type IxAccounts = FuzzAccounts;
}
