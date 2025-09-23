use anchor_lang::prelude::*;

use crate::constants::DISCRIMINATOR;

#[account]
#[derive(InitSpace)]
pub struct Lottery {
    pub id: u64,
    pub collection_mint: Pubkey,
    pub prize: u64,
    pub start: i64,
    pub end: i64,
    pub winner: u64,
    pub ticket_price: u64,
    pub total_tickets: u64,
    pub bump: u8,
}

impl Lottery {
    pub const LEN: usize = DISCRIMINATOR + Lottery::INIT_SPACE;
}
