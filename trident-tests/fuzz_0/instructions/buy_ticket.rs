use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("B6VvVi2nqC5vQfDUxEjcBNCHN7r1kBMZGSAAJyUg3e2j")]
#[discriminator([11u8, 24u8, 17u8, 193u8, 168u8, 116u8, 164u8, 169u8])]
pub struct BuyTicketInstruction {
    pub accounts: BuyTicketInstructionAccounts,
    pub data: BuyTicketInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(BuyTicketInstructionData)]
#[storage(FuzzAccounts)]
pub struct BuyTicketInstructionAccounts {
    #[account(mut, signer)]
    pub buyer: TridentAccount,

    #[account(mut)]
    pub global_config: TridentAccount,

    #[account(mut)]
    pub treasury: TridentAccount,

    #[account(mut)]
    pub lottery: TridentAccount,

    #[account(mut)]
    pub collection_mint: TridentAccount,

    #[account(mut)]
    pub mint: TridentAccount,

    #[account(mut)]
    pub buyer_token_account: TridentAccount,

    #[account(mut)]
    pub metadata_account: TridentAccount,

    #[account(mut)]
    pub edition_account: TridentAccount,

    #[account(address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")]
    pub token_metadata_program: TridentAccount,

    #[account(address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")]
    pub associated_token_program: TridentAccount,

    pub token_program: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,

    #[account(address = "SysvarRent111111111111111111111111111111111")]
    pub rent: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct BuyTicketInstructionData {
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
impl InstructionHooks for BuyTicketInstruction {
    type IxAccounts = FuzzAccounts;
}
