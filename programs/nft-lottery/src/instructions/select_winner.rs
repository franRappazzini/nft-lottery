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
        has_one = randomness_account @ DappError::InvalidRandomnessAccount,
        constraint = !lottery.winner_chosen @ DappError::WinnerAlreadySelected,
    )]
    pub lottery: Account<'info, Lottery>,

    /// CHECK This account is checked by Switchboard
    pub randomness_account: UncheckedAccount<'info>,
}

impl<'info> SelectWinner<'info> {
    pub fn select_winner(&mut self, _lottery_id: u64) -> Result<()> {
        // Load clock to check data from the future
        let clock = Clock::get()?;

        // Update player_state's randomness_account
        let randomness_data =
            RandomnessAccountData::parse(self.randomness_account.data.borrow()).unwrap();

        let revealed_random_value = randomness_data
            .get_value(clock.slot)
            .map_err(|_| DappError::RandomnessNotResolved)?;

        let winner = revealed_random_value[0] as u64 % self.lottery.total_tickets;

        msg!("Winner: {:?}", winner);

        self.lottery.winner = winner;
        self.lottery.winner_chosen = true;
        self.lottery.end = clock.unix_timestamp;

        Ok(())
    }
}
