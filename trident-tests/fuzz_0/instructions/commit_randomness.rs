use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("B6VvVi2nqC5vQfDUxEjcBNCHN7r1kBMZGSAAJyUg3e2j")]
#[discriminator([146u8, 52u8, 195u8, 220u8, 79u8, 30u8, 53u8, 26u8])]
pub struct CommitRandomnessInstruction {
    pub accounts: CommitRandomnessInstructionAccounts,
    pub data: CommitRandomnessInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(CommitRandomnessInstructionData)]
#[storage(FuzzAccounts)]
pub struct CommitRandomnessInstructionAccounts {
    #[account(mut, signer)]
    pub signer: TridentAccount,

    #[account(mut)]
    pub lottery: TridentAccount,

    pub randomness_account: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct CommitRandomnessInstructionData {
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
impl InstructionHooks for CommitRandomnessInstruction {
    type IxAccounts = FuzzAccounts;
}
