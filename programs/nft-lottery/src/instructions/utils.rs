use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        self,
        mpl_token_metadata::types::{Collection, CollectionDetails, Creator, DataV2},
        Metadata,
    },
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::states::{GlobalConfig, Lottery};

pub fn create_mint<'info>(
    authority: &AccountInfo<'info>,
    mint: &InterfaceAccount<'info, Mint>,
    to: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Interface<'info, TokenInterface>,
    signer_seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    let cpi_accounts = token_interface::MintTo {
        authority: authority.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
    };

    let cpi_ctx = if let Some(signer_seeds) = signer_seeds {
        CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, signer_seeds)
    } else {
        CpiContext::new(token_program.to_account_info(), cpi_accounts)
    };

    token_interface::mint_to(cpi_ctx, 1)?;
    Ok(())
}

pub fn create_metadata_accounts<'info>(
    metadata: &UncheckedAccount<'info>,
    mint: &InterfaceAccount<'info, Mint>,
    mint_authority: &AccountInfo<'info>,
    payer: &Signer<'info>,
    update_authority: &AccountInfo<'info>,
    system_program: &Program<'info, System>,
    rent: &Sysvar<'info, Rent>,
    token_metadata_program: &Program<'info, Metadata>,
    global_config: &Account<'info, GlobalConfig>,
    collection_mint: &InterfaceAccount<'info, Mint>,
    lottery: Option<&Account<'info, Lottery>>,
    signer_seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    let cpi_accounts = metadata::CreateMetadataAccountsV3 {
        metadata: metadata.to_account_info(),
        mint: mint.to_account_info(),
        mint_authority: mint_authority.to_account_info(),
        payer: payer.to_account_info(),
        update_authority: update_authority.to_account_info(),
        system_program: system_program.to_account_info(),
        rent: rent.to_account_info(),
    };

    let cpi_ctx = if let Some(signer_seeds) = signer_seeds {
        CpiContext::new_with_signer(
            token_metadata_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        )
    } else {
        CpiContext::new(token_metadata_program.to_account_info(), cpi_accounts)
    };

    let is_collection = lottery.is_none();

    let name = if is_collection {
        format!("NFT Lottery {}", global_config.next_lottery_id)
    } else {
        format!(
            "NFT Lottery {} #{}",
            global_config.next_lottery_id,
            lottery.unwrap().total_tickets
        )
    };

    let data = DataV2 {
            collection: if is_collection {
                None
            } else {
                Some(Collection {
                    verified: false,
                    key: collection_mint.key(),
                })
            },
            name,
            symbol: "LTRY".to_string(),
            uri: "https://plus.unsplash.com/premium_photo-1718191345906-39c178e43133?fm=jpg&q=60&w=3000&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxzZWFyY2h8MXx8bG90dGVyeSUyMHdpbm5lcnxlbnwwfHwwfHx8MA%3D%3D".to_string(),
            seller_fee_basis_points: 0,
            creators: Some(vec![Creator{
                address: global_config.key(),
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
        if is_collection {
            Some(CollectionDetails::V1 { size: 0 })
        } else {
            None
        },
    )
}

pub fn create_master_edition<'info>(
    edition_account: &UncheckedAccount<'info>,
    mint: &InterfaceAccount<'info, Mint>,
    update_authority: &AccountInfo<'info>,
    mint_authority: &AccountInfo<'info>,
    payer: &Signer<'info>,
    metadata: &UncheckedAccount<'info>,
    token_metadata_program: &Program<'info, Metadata>,
    token_program: &Interface<'info, TokenInterface>,
    system_program: &Program<'info, System>,
    rent: &Sysvar<'info, Rent>,
    signer_seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    let cpi_accounts = metadata::CreateMasterEditionV3 {
        edition: edition_account.to_account_info(),
        mint: mint.to_account_info(),
        update_authority: update_authority.to_account_info(),
        mint_authority: mint_authority.to_account_info(),
        payer: payer.to_account_info(),
        metadata: metadata.to_account_info(),
        token_program: token_program.to_account_info(),
        system_program: system_program.to_account_info(),
        rent: rent.to_account_info(),
    };

    let cpi_ctx = if let Some(signer_seeds) = signer_seeds {
        CpiContext::new_with_signer(
            token_metadata_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        )
    } else {
        CpiContext::new(token_metadata_program.to_account_info(), cpi_accounts)
    };

    metadata::create_master_edition_v3(cpi_ctx, Some(0))
}

pub fn sign_metadata<'info>(
    metadata: &UncheckedAccount<'info>,
    creator: &AccountInfo<'info>,
    token_metadata_program: &Program<'info, Metadata>,
    signer_seeds: Option<&[&[&[u8]]]>,
) -> Result<()> {
    let cpi_accounts = metadata::SignMetadata {
        metadata: metadata.to_account_info(),
        creator: creator.to_account_info(),
    };

    let cpi_ctx = if let Some(signer_seeds) = signer_seeds {
        CpiContext::new_with_signer(
            token_metadata_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        )
    } else {
        CpiContext::new(token_metadata_program.to_account_info(), cpi_accounts)
    };

    metadata::sign_metadata(cpi_ctx)
}
