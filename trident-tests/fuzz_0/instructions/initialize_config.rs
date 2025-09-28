use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("B6VvVi2nqC5vQfDUxEjcBNCHN7r1kBMZGSAAJyUg3e2j")]
#[discriminator([208u8, 127u8, 21u8, 1u8, 194u8, 190u8, 196u8, 70u8])]
pub struct InitializeConfigInstruction {
    pub accounts: InitializeConfigInstructionAccounts,
    pub data: InitializeConfigInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeConfigInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeConfigInstructionAccounts {
    #[account(
        mut,
        signer,
        storage::name = authority,
        storage::account_id = (0..1),
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

    #[account(address = "SysvarRent111111111111111111111111111111111")]
    pub rent: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeConfigInstructionData {
    pub fee_bps: u16,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeConfigInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        self.data.fee_bps = trident.gen_range(0..u16::MAX)
    }
}
