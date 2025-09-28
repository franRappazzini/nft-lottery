use anchor_lang::prelude::*;

use crate::constants::DISCRIMINATOR;

#[account]
#[derive(InitSpace)]
pub struct GlobalConfig {
    pub authority: Pubkey,
    pub next_lottery_id: u64,
    pub treasury: Pubkey,
    pub collected_fees: u64,
    pub fee_bps: u16,
    pub bump: u8,
}

impl GlobalConfig {
    pub const LEN: usize = DISCRIMINATOR + GlobalConfig::INIT_SPACE;
}
