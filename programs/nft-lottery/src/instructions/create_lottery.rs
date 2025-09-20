use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        self,
        mpl_token_metadata::types::{CollectionDetails, Creator, DataV2},
        Metadata,
    },
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{COLLECTION_MINT_SEED, COLLECTION_TOKEN_ACCOUNT_SEED, GLOBAL_CONFIG_SEED},
    states::GlobalConfig,
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
    pub fn create_lottery(&mut self) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[GLOBAL_CONFIG_SEED, &[self.global_config.bump]]];

        self.mint_collection(signer_seeds)?;

        self.create_metadata_accounts(signer_seeds)?;

        self.create_master_edition(signer_seeds)?;

        self.sign_metadata(signer_seeds)?;

        self.global_config.next_lottery_id += 1;

        Ok(())
    }

    pub fn mint_collection(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let cpi_accounts = token_interface::MintTo {
            authority: self.global_config.to_account_info(),
            mint: self.collection_mint.to_account_info(),
            to: self.collection_token_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        token_interface::mint_to(cpi_ctx, 1)
    }

    pub fn create_metadata_accounts(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let cpi_accounts = metadata::CreateMetadataAccountsV3 {
            metadata: self.metadata_account.to_account_info(),
            mint: self.collection_mint.to_account_info(),
            mint_authority: self.global_config.to_account_info(),
            payer: self.creator.to_account_info(),
            update_authority: self.global_config.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_metadata_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        let data = DataV2 {
            collection: None,
            name: format!("NFT Lottery #{}", self.global_config.next_lottery_id),
            symbol: "LTRY".to_string(),
            uri: "https://plus.unsplash.com/premium_photo-1718191345906-39c178e43133?fm=jpg&q=60&w=3000&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MXx8bG90dGVyeSUyMHdpbm5lcnxlbnwwfHwwfHx8MA%3D%3D".to_string(),
            seller_fee_basis_points: 0,
            creators:Some(vec![Creator{
                address: self.global_config.key(),
                verified: true,
                share: 100,
            }]),
            uses: None,
        };

        metadata::create_metadata_accounts_v3(
            cpi_ctx,
            data,
            false,
            false,
            Some(CollectionDetails::V1 { size: 0 }),
        )
    }

    pub fn create_master_edition(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let cpi_accounts = metadata::CreateMasterEditionV3 {
            edition: self.edition_account.to_account_info(),
            mint: self.collection_mint.to_account_info(),
            update_authority: self.global_config.to_account_info(),
            mint_authority: self.global_config.to_account_info(),
            payer: self.creator.to_account_info(),
            metadata: self.metadata_account.to_account_info(),
            token_program: self.token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_metadata_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        metadata::create_master_edition_v3(cpi_ctx, Some(0))
    }

    pub fn sign_metadata(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let cpi_accounts = metadata::SignMetadata {
            metadata: self.metadata_account.to_account_info(),
            creator: self.global_config.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_metadata_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        metadata::sign_metadata(cpi_ctx)
    }
}
