use anchor_lang::{prelude::*, system_program};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    constants::{GLOBAL_CONFIG_SEED, LOTTERY_SEED, TICKET_MINT_SEED, TREASURY_SEED},
    errors::DappError,
    states::{GlobalConfig, Lottery},
};

#[derive(Accounts)]
#[instruction(lottery_id: u64)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
    pub claimer: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        address = global_config.treasury,
        seeds = [TREASURY_SEED],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        seeds = [LOTTERY_SEED, lottery_id.to_le_bytes().as_ref()],
        bump = lottery.bump,
        constraint = lottery.winner_chosen @ DappError::WinnerNotSelected,
        constraint = !lottery.prize_claimed @ DappError::PrizeAlreadyClaimed,
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(
        seeds = [
            TICKET_MINT_SEED,
            lottery_id.to_le_bytes().as_ref(),
            lottery.winner.to_le_bytes().as_ref()
        ],
        bump,
    )]
    pub ticket_mint: InterfaceAccount<'info, Mint>,

    #[account(
        associated_token::mint = ticket_mint,
        associated_token::authority = claimer,
        associated_token::token_program = token_program,
        constraint = claimer_token_account.amount > 0 @ DappError::NoTicketOwned,
    )]
    pub claimer_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimPrize<'info> {
    pub fn claim_prize(&mut self, _lottery_id: u64, treasury_bump: u8) -> Result<()> {
        let prize_amount = self.lottery.ticket_price * self.lottery.total_tickets;
        let fee_amount = (prize_amount * self.global_config.fee_bps as u64) / 10_000;
        let winner_amount = prize_amount - fee_amount;

        msg!(
            "Fee amount: {}. Winner amount: {}",
            fee_amount,
            winner_amount
        );

        self.global_config.collected_fees += fee_amount;
        self.lottery.prize_claimed = true;

        let signer_seeds: &[&[&[u8]]] = &[&[TREASURY_SEED, &[treasury_bump]]];

        let cpi_accounts = system_program::Transfer {
            from: self.treasury.to_account_info(),
            to: self.claimer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        system_program::transfer(cpi_ctx, winner_amount)
    }
}
