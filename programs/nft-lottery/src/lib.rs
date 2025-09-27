mod constants;
mod errors;
mod instructions;
mod states;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("4N47FFTLdp2kLqHNxS2VRndT4Mq2yjnRpFhi98phUkca");

#[program]
pub mod nft_lottery {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>, fee_bps: u16) -> Result<()> {
        ctx.accounts
            .initialize_config(fee_bps, ctx.bumps.global_config)
    }

    pub fn create_lottery(
        ctx: Context<CreateLottery>,
        ticket_price: u64,
        max_tickets: u64,
    ) -> Result<()> {
        ctx.accounts
            .create_lottery(ticket_price, max_tickets, ctx.bumps.lottery)
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, lottery_id: u64) -> Result<()> {
        ctx.accounts.buy_ticket(lottery_id)
    }

    pub fn commit_randomness(ctx: Context<CommitRandomness>, lottery_id: u64) -> Result<()> {
        ctx.accounts.commit_randomness(lottery_id)
    }

    pub fn select_winner(ctx: Context<SelectWinner>, lottery_id: u64) -> Result<()> {
        ctx.accounts.select_winner(lottery_id)
    }
}
