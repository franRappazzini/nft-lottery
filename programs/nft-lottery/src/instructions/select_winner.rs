use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

use crate::{
    constants::{GLOBAL_CONFIG_SEED, LOTTERY_SEED},
    errors::DappError,
    states::{GlobalConfig, Lottery},
};

#[derive(Accounts)]
#[instruction(lottery_id: u64)]
pub struct SelectWinner<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_state.bump,
    )]
    pub global_state: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [LOTTERY_SEED, lottery_id.to_le_bytes().as_ref()],
        bump = lottery.bump,
        // constraint = lottery.total_tickets == lottery.max_tickets @ ErrorCode::LotteryNotFull,
        // constraint = lottery.winner.is_none() @ ErrorCode::WinnerAlreadySelected,
    )]
    pub lottery: Account<'info, Lottery>,

    /// CHECK This account is checked by Switchboard
    pub randomness_account: UncheckedAccount<'info>,
}

impl<'info> SelectWinner<'info> {
    pub fn select_winner(&mut self, _lottery_id: u64) -> Result<()> {
        // Load clock to check data from the future
        let clock = Clock::get()?;

        require!(clock.slot < self.lottery.end as u64, DappError::OutOfTime);

        // Update player_state's randomness_account
        let randomness_data =
            RandomnessAccountData::parse(self.randomness_account.data.borrow()).unwrap();

        let revealed_random_value = randomness_data
            .get_value(clock.slot)
            .map_err(|_| DappError::RandomnessNotResolved)?;

        msg!("Random value: {:?}", revealed_random_value);

        Ok(())
    }
}
