use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{self, mpl_token_metadata::types::{Collection, Creator, DataV2}, Metadata},
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{COLLECTION_MINT_SEED, GLOBAL_CONFIG_SEED, LOTTERY_SEED, TICKET_MINT_SEED, TREASURY_SEED},
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

        msg!("Minted ticket NFT to: {}", self.buyer_token_account.key());

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

        msg!(
            "Created metadata account for: {}",
            self.buyer_token_account.key()
        );

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

        msg!(
            "Created master edition account for: {}",
            self.buyer_token_account.key()
        );

        utils::sign_metadata(
            &self.metadata_account,
            &self.global_config.to_account_info(),
            &self.token_metadata_program,
            Some(signer_seeds),
        )?;

        msg!(
            "Signed metadata account for: {}",
            self.buyer_token_account.key()
        );

        self.lottery.total_tickets += 1;
        self.lottery.prize += self.lottery.ticket_price;

        Ok(())
    }

    pub fn buy_ticket2(&mut self, lottery_id: u64, collection_mint_bump: u8) -> Result<()> {
        let cpi_accounts = system_program::Transfer {
            from: self.buyer.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        system_program::transfer(cpi_ctx, self.lottery.ticket_price)?;

        // let signer_seeds: &[&[&[u8]]] = &[&[COLLECTION_MINT_SEED, &lottery_id.to_le_bytes(), &[collection_mint_bump]]];
        let signer_seeds: &[&[&[u8]]] = &[&[GLOBAL_CONFIG_SEED, &[self.global_config.bump]]];

        let cpi_accounts = token_interface::MintTo {
            authority: self.global_config.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.buyer_token_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        token_interface::mint_to(cpi_ctx, 1)?;

        msg!("Minted ticket NFT to: {}", self.buyer_token_account.key());
        let cpi_accounts = metadata::CreateMetadataAccountsV3 {
            metadata: self.metadata_account.to_account_info(),
            mint: self.mint.to_account_info(),
            mint_authority: self.global_config.to_account_info(),
            payer: self.buyer.to_account_info(),
            update_authority: self.global_config.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };

        let cpi_ctx = 
            CpiContext::new_with_signer(
                self.token_metadata_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
        );




    let data = DataV2 {
            collection: 
                Some(Collection {
                    verified: false,
                    key: self.collection_mint.key(),
                })
            ,
            name:      format!(
            "NFT Lottery {} #{}",
           self.global_config.next_lottery_id,
           self.lottery.total_tickets
        ),
            symbol: "LTRY".to_string(),
            uri: "https://plus.unsplash.com/premium_photo-1718191345906-39c178e43133?fm=jpg&q=60&w=3000&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MXx8bG90dGVyeSUyMHdpbm5lcnxlbnwwfHwwfHx8MA%3D%3D".to_string(),
            seller_fee_basis_points: 0,
            creators: Some(vec![Creator{
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
            None

    )
?;

        msg!(
            "Created metadata account for: {}",
            self.buyer_token_account.key()
        );

      let cpi_accounts = metadata::CreateMasterEditionV3 {
        edition: self.edition_account.to_account_info(),
        mint: self.mint.to_account_info(),
        update_authority: self.global_config.to_account_info(),
        mint_authority: self.global_config.to_account_info(),
        payer: self.buyer.to_account_info(),
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

    metadata::create_master_edition_v3(cpi_ctx, Some(0))?;

        msg!(
            "Created master edition account for: {}",
            self.buyer_token_account.key()
        );

       let cpi_accounts = metadata::SignMetadata {
        metadata: self.metadata_account.to_account_info(),
        creator: self.global_config.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        self.token_metadata_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

    metadata::sign_metadata(cpi_ctx)?;
        msg!(
            "Signed metadata account for: {}",
            self.buyer_token_account.key()
        );

        self.lottery.total_tickets += 1;
        self.lottery.prize += self.lottery.ticket_price;

        Ok(())
    }
}
