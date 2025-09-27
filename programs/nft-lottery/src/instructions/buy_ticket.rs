use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{
        COLLECTION_MINT_SEED, GLOBAL_CONFIG_SEED, LOTTERY_SEED, TICKET_MINT_SEED, TREASURY_SEED,
    },
    errors::DappError,
    instructions::utils,
    states::{GlobalConfig, Lottery},
};

#[derive(Accounts)]
#[instruction(lottery_id: u64)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [TREASURY_SEED],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [LOTTERY_SEED, lottery_id.to_le_bytes().as_ref()],
        bump = lottery.bump,
        constraint = lottery.total_tickets < lottery.max_tickets @ DappError::MaxTicketsReached,
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(
        mut,
        seeds = [COLLECTION_MINT_SEED, lottery_id.to_le_bytes().as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = buyer,
        mint::decimals = 0,
        mint::authority = global_config,
        mint::freeze_authority = global_config,
        seeds = [TICKET_MINT_SEED, lottery_id.to_le_bytes().as_ref(), lottery.total_tickets.to_le_bytes().as_ref()],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
        associated_token::token_program = token_program,
    )]
    pub buyer_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub edition_account: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> BuyTicket<'info> {
    pub fn buy_ticket(&mut self, _lottery_id: u64) -> Result<()> {
        let cpi_accounts = system_program::Transfer {
            from: self.buyer.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        system_program::transfer(cpi_ctx, self.lottery.ticket_price)?;

        let signer_seeds: &[&[&[u8]]] = &[&[GLOBAL_CONFIG_SEED, &[self.global_config.bump]]];

        utils::create_mint(
            &self.global_config.to_account_info(),
            &self.mint,
            &self.buyer_token_account,
            &self.token_program,
            Some(signer_seeds),
        )?;

        utils::create_metadata_accounts(
            &self.metadata_account,
            &self.mint,
            &self.global_config.to_account_info(),
            &self.buyer,
            &self.global_config.to_account_info(),
            &self.system_program,
            &self.rent,
            &self.token_metadata_program,
            &self.global_config,
            &self.collection_mint,
            Some(&self.lottery),
            Some(signer_seeds),
        )?;

        utils::create_master_edition(
            &self.edition_account,
            &self.mint,
            &self.global_config.to_account_info(),
            &self.global_config.to_account_info(),
            &self.buyer,
            &self.metadata_account,
            &self.token_metadata_program,
            &self.token_program,
            &self.system_program,
            &self.rent,
            Some(signer_seeds),
        )?;

        utils::sign_metadata(
            &self.metadata_account,
            &self.global_config.to_account_info(),
            &self.token_metadata_program,
            Some(signer_seeds),
        )?;

        self.lottery.total_tickets += 1;
        self.lottery.prize += self.lottery.ticket_price;

        Ok(())
    }
}
