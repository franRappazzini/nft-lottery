use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("B6VvVi2nqC5vQfDUxEjcBNCHN7r1kBMZGSAAJyUg3e2j")]
#[discriminator([242u8, 165u8, 247u8, 119u8, 17u8, 203u8, 21u8, 42u8])]
pub struct CreateLotteryInstruction {
    pub accounts: CreateLotteryInstructionAccounts,
    pub data: CreateLotteryInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(CreateLotteryInstructionData)]
#[storage(FuzzAccounts)]
pub struct CreateLotteryInstructionAccounts {
    #[account(mut, signer)]
    pub creator: TridentAccount,

    #[account(mut)]
    pub global_config: TridentAccount,

    #[account(mut)]
    pub lottery: TridentAccount,

    #[account(mut)]
    pub collection_mint: TridentAccount,

    #[account(mut)]
    pub collection_token_account: TridentAccount,

    #[account(mut)]
    pub metadata_account: TridentAccount,

    #[account(mut)]
    pub edition_account: TridentAccount,

    #[account(address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")]
    pub token_metadata_program: TridentAccount,

    pub token_program: TridentAccount,

    #[account(address = "SysvarRent111111111111111111111111111111111")]
    pub rent: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct CreateLotteryInstructionData {
    pub ticket_price: u64,

    pub max_tickets: u64,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for CreateLotteryInstruction {
    type IxAccounts = FuzzAccounts;
}
