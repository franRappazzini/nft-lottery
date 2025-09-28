use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct GlobalConfig {
    pub authority: TridentPubkey,

    pub next_lottery_id: u64,

    pub treasury: TridentPubkey,

    pub collected_fees: u64,

    pub fee_bps: u16,

    pub bump: u8,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct Lottery {
    pub id: u64,

    pub collection_mint: TridentPubkey,

    pub randomness_account: TridentPubkey,

    pub prize: u64,

    pub start: i64,

    pub end: i64,

    pub winner: u64,

    pub winner_chosen: bool,

    pub ticket_price: u64,

    pub total_tickets: u64,

    pub max_tickets: u64,

    pub prize_claimed: bool,

    pub bump: u8,
}
