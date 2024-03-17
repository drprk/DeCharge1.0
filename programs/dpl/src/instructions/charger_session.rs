use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{constants::DEFAULT_TOKEN_MINT, state::charger::Charger};

pub fn charger_session(ctx: Context<ChargerSession>) -> Result<()> {
    let mint = &ctx.accounts.mint;

    require!(
        mint.key() == Pubkey::from_str(DEFAULT_TOKEN_MINT).unwrap(),
        crate::errors::DplError::InvalidMint
    );

    // transfer from user ata to nft mint owner ata (70%) and operator ata (30%), take amount as an argument or take some time as an argument and make like 1 minute = 0.5 usdc
    // update the all time revenue in charger pda

    Ok(())
}

#[derive(Accounts)]
pub struct ChargerSession<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        token::mint = mint,
        token::authority = user
    )]
    pub user_ata: Account<'info, TokenAccount>,
    pub charger: AccountInfo<'info>,
    #[account(
        seeds = [b"charger", charger.key().as_ref()],
        bump
    )]
    pub charger_pda: Account<'info, Charger>,
    pub mint: Account<'info, Mint>,
    pub nft_mint: Account<'info, Mint>,
    pub nft_mint_owner: AccountInfo<'info>,
    #[account(
        token::mint = mint,
        token::authority = nft_mint_owner
    )]
    pub nft_mint_owner_ata: Account<'info, TokenAccount>,
    pub operator: AccountInfo<'info>,
    #[account(
        token::mint = mint,
        token::authority = operator
    )]
    pub operator_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
