use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{constants::DEFAULT_TOKEN_MINT, state::charger::Charger};

pub fn create_charger_ix(ctx: Context<CreateCharger>) -> Result<()> {
    let charger = &ctx.accounts.charger;
    let charger_pda = &mut ctx.accounts.charger_pda;
    let mint = &ctx.accounts.mint;
    let nft_mint = &ctx.accounts.nft_mint;

    require!(
        mint.key() == Pubkey::from_str(DEFAULT_TOKEN_MINT).unwrap(),
        crate::errors::DplError::InvalidMint
    );

    let clock: Clock = Clock::get()?;

    charger_pda.pubkey = *charger.key;
    charger_pda.created_at = clock.unix_timestamp;
    charger_pda.nft_mint = nft_mint.key();
    charger_pda.all_time_revenue = 0;
    charger_pda.revenue_to_pay = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateCharger<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: identifier
    pub charger: AccountInfo<'info>,
    #[account(
        init,
        seeds = [b"charger", charger.key().as_ref()],
        bump,
        payer = payer,
        space = Charger::LEN,
    )]
    pub charger_pda: Account<'info, Charger>,
    #[account(
        init,
        payer = payer,
        seeds = [b"vault".as_ref(), charger.key().as_ref(), mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = charger_pda
    )]
    pub vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub nft_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
