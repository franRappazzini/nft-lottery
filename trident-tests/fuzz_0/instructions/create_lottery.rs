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
    #[account(
        mut,
        signer,
        storage::name = creator,
        storage::account_id = (0..1)
    )]
    pub creator: TridentAccount,

    #[account(
        mut,
        storage::name = global_config,
        storage::account_id = (0..1),
        seeds = [b"global_config"],
    )]
    pub global_config: TridentAccount,

    #[account(
        mut,
        storage::name = lottery,
        storage::account_id = (0..1),
        seeds = [b"lottery", 0u64.to_le_bytes().as_ref()],
    )]
    pub lottery: TridentAccount,

    #[account(
        mut,
        storage::name = collection_mint,
        storage::account_id = (0..1),
        seeds = [b"collection_mint", 0u64.to_le_bytes().as_ref()],
    )]
    pub collection_mint: TridentAccount,

    #[account(
        mut,
        storage::name = collection_token_account,
        storage::account_id = (0..1),
        seeds = [b"collection_token_account", 0u64.to_le_bytes().as_ref()],
    )]
    pub collection_token_account: TridentAccount,

    #[account(
        mut,
        storage::name = metadata_account,
        storage::account_id = (0..1),
        seeds = [
            b"metadata",
            token_metadata_program.as_ref(),
            collection_mint.as_ref()
        ],
        program_id = token_metadata_program,
    )]
    pub metadata_account: TridentAccount,

    #[account(
        mut,
        storage::name = edition_account,
        storage::account_id = (0..1),
        seeds = [
            b"metadata",
            token_metadata_program.as_ref(),
            collection_mint.as_ref(),
            b"edition"
        ],
        program_id = token_metadata_program,
    )]
    pub edition_account: TridentAccount,

    #[account(address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")]
    pub token_metadata_program: TridentAccount,

    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
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

    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        self.data.max_tickets = trident.gen_range(0..u64::MAX);
        self.data.ticket_price = trident.gen_range(0..u64::MAX);
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {

        // // Generate random account id
        // let account_id = trident.gen_range(0..1);

        // // Create and store a PDA
        // let id = &trident.gen_range(0..u64::MAX);

        // println!("Creating lottery with id: {}", id);
        // let lottery_account = fuzz_accounts.lottery.get_or_create(
        //     account_id,
        //     trident,
        //     Some(PdaSeeds::new(
        //         &[b"lottery", id.to_le_bytes().as_ref()],
        //         self.get_program_id(),
        //     )),
        //     None,
        // );

        // // Set the account address
        // self.accounts.lottery.set_address(lottery_account);

        // let collection_mint_account = fuzz_accounts.collection_mint.get_or_create(
        //     account_id,
        //     trident,
        //     Some(PdaSeeds::new(
        //         &[b"collection_mint", id.to_le_bytes().as_ref()],
        //         self.get_program_id(),
        //     )),
        //     None,
        // );

        // self.accounts
        //     .collection_mint
        //     .set_address(collection_mint_account);

        // let collection_token_account = fuzz_accounts.collection_token_account.get_or_create(
        //     account_id,
        //     trident,
        //     Some(PdaSeeds::new(
        //         &[b"collection_token_account", id.to_le_bytes().as_ref()],
        //         self.get_program_id(),
        //     )),
        //     None,
        // );

        // self.accounts
        //     .collection_token_account
        //     .set_address(collection_token_account);
    }
}
