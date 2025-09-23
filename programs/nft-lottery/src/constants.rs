use anchor_lang::constant;

pub const DISCRIMINATOR: usize = 8;

#[constant]
pub const GLOBAL_CONFIG_SEED: &[u8] = b"global_config";

#[constant]
pub const TREASURY_SEED: &[u8] = b"treasury";

#[constant]
pub const COLLECTION_MINT_SEED: &[u8] = b"collection_mint";

#[constant]
pub const COLLECTION_TOKEN_ACCOUNT_SEED: &[u8] = b"collection_token_account";

#[constant]
pub const LOTTERY_SEED: &[u8] = b"lottery";

#[constant]
pub const TICKET_MINT_SEED: &[u8] = b"ticket_mint";
