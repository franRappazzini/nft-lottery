use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::Metadata,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{
        COLLECTION_MINT_SEED, COLLECTION_TOKEN_ACCOUNT_SEED, GLOBAL_CONFIG_SEED, LOTTERY_SEED,
    },
    instructions::utils,
    states::{GlobalConfig, Lottery},
};

#[derive(Accounts)]
pub struct CreateLottery<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_CONFIG_SEED],
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        init,
        payer = creator,
        space = Lottery::LEN,
        seeds = [LOTTERY_SEED, global_config.next_lottery_id.to_le_bytes().as_ref()],
        bump
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(
        init,
        payer = creator,
        mint::decimals = 0,
        mint::authority = global_config,
        mint::freeze_authority = global_config,
        seeds = [COLLECTION_MINT_SEED, global_config.next_lottery_id.to_le_bytes().as_ref()],
        bump
    )]
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = creator,
        token::mint = collection_mint,
        token::authority = global_config,
        seeds = [COLLECTION_TOKEN_ACCOUNT_SEED, global_config.next_lottery_id.to_le_bytes().as_ref()],
        bump
    )]
    pub collection_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref()
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
            collection_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub edition_account: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Interface<'info, TokenInterface>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateLottery<'info> {
    pub fn create_lottery(
        &mut self,
        ticket_price: u64,
        max_tickets: u64,
        lottery_bump: u8,
    ) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[GLOBAL_CONFIG_SEED, &[self.global_config.bump]]];

        utils::create_mint(
            &self.global_config.to_account_info(),
            &self.collection_mint,
            &self.collection_token_account,
            &self.token_program,
            Some(signer_seeds),
        )?;

        utils::create_metadata_accounts(
            &self.metadata_account,
            &self.collection_mint,
            &self.global_config.to_account_info(),
            &self.creator,
            &self.global_config.to_account_info(),
            &self.system_program,
            &self.rent,
            &self.token_metadata_program,
            &self.global_config,
            &self.collection_mint,
            None,
            Some(signer_seeds),
        )?;

        utils::create_master_edition(
            &self.edition_account,
            &self.collection_mint,
            &self.global_config.to_account_info(),
            &self.global_config.to_account_info(),
            &self.creator,
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

        self.lottery.set_inner(Lottery {
            id: self.global_config.next_lottery_id,
            collection_mint: self.collection_mint.key(),
            randomness_account: Pubkey::default(),
            prize: 0,
            start: Clock::get()?.unix_timestamp,
            end: 0,
            winner: 0,
            winner_chosen: false,
            ticket_price,
            total_tickets: 0,
            max_tickets,
            prize_claimed: false,
            bump: lottery_bump,
        });

        self.global_config.next_lottery_id += 1;

        Ok(())
    }
}
