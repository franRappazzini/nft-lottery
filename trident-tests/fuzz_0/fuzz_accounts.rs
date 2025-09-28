use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub global_state: AccountsStorage,

    pub claimer_token_account: AccountsStorage,

    pub token_program: AccountsStorage,

    pub global_config: AccountsStorage,

    pub treasury: AccountsStorage,

    pub rent: AccountsStorage,

    pub ticket_mint: AccountsStorage,

    pub system_program: AccountsStorage,

    pub randomness_account: AccountsStorage,

    pub creator: AccountsStorage,

    pub buyer_token_account: AccountsStorage,

    pub collection_token_account: AccountsStorage,

    pub associated_token_program: AccountsStorage,

    pub lottery: AccountsStorage,

    pub edition_account: AccountsStorage,

    pub mint: AccountsStorage,

    pub token_metadata_program: AccountsStorage,

    pub claimer: AccountsStorage,

    pub signer: AccountsStorage,

    pub buyer: AccountsStorage,

    pub authority: AccountsStorage,

    pub collection_mint: AccountsStorage,

    pub metadata_account: AccountsStorage,
}
