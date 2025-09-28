use anchor_lang::{
    prelude::*,
    system_program::{self},
};

use crate::{
    constants::{GLOBAL_CONFIG_SEED, TREASURY_SEED},
    states::GlobalConfig,
};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = GlobalConfig::LEN,
        seeds = [GLOBAL_CONFIG_SEED],
        bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [TREASURY_SEED],
        bump,
    )]
    pub treasury: SystemAccount<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(&mut self, fee_bps: u16, bump: u8) -> Result<()> {
        self.global_config.set_inner(GlobalConfig {
            authority: self.authority.key(),
            next_lottery_id: 0,
            treasury: self.treasury.key(),
            collected_fees: 0,
            fee_bps,
            bump,
        });

        let min_rent = self.rent.minimum_balance(self.treasury.data_len());

        let cpi_accounts = system_program::Transfer {
            from: self.authority.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        system_program::transfer(cpi_ctx, min_rent)
    }
}
