use anchor_lang::{prelude::*, system_program};

use crate::{
    constants::{GLOBAL_CONFIG_SEED, TREASURY_SEED},
    errors::DappError,
    states::GlobalConfig,
};

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
        has_one = authority,
        constraint = global_config.collected_fees > 0 @ DappError::NoFees
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        address = global_config.treasury,
        seeds = [TREASURY_SEED],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawFees<'info> {
    pub fn withdraw_fees(&mut self, treasury_bump: u8) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[TREASURY_SEED, &[treasury_bump]]];

        let cpi_accounts = system_program::Transfer {
            from: self.treasury.to_account_info(),
            to: self.authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        system_program::transfer(cpi_ctx, self.global_config.collected_fees)?;

        self.global_config.collected_fees = 0;

        Ok(())
    }
}
