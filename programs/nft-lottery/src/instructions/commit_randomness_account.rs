use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

use crate::{constants::LOTTERY_SEED, errors::DappError, states::Lottery};

#[derive(Accounts)]
#[instruction(lottery_id: u64)]
pub struct CommitRandomness<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [LOTTERY_SEED, lottery_id.to_le_bytes().as_ref()],
        bump = lottery.bump,
        // has_one = authority @ CustomErrors::UnauthorizedAction
    )]
    pub lottery: Account<'info, Lottery>,

    /// CHECK: This account is checked by Switchboard
    pub randomness_account: UncheckedAccount<'info>,
}

impl<'info> CommitRandomness<'info> {
    pub fn commit_randomness(&mut self, _lottery_id: u64) -> Result<()> {
        // https://docs.switchboard.xyz/product-documentation/randomness/tutorials/solana-svm
        // Load clock to check data from the future
        let clock = Clock::get()?;

        // Update token_lottery's randomness_account
        let randomness_data =
            RandomnessAccountData::parse(self.randomness_account.data.borrow()).unwrap();

        require!(
            randomness_data.seed_slot == clock.slot - 1,
            DappError::RandomnessAlreadyRevealed
        );

        self.lottery.randomness_account = self.randomness_account.key();

        Ok(())
    }
}
