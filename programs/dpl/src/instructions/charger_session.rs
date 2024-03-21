use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked};

use crate::{constants::DEFAULT_TOKEN_MINT, errors::DplError, state::charger::Charger};

pub fn charger_session_ix(ctx: Context<ChargerSession>, amount: u64) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let user = &ctx.accounts.user;
    let user_ata = &ctx.accounts.user_ata;
    let operator_ata = &ctx.accounts.operator_ata;
    let nft_mint_owner_ata = &ctx.accounts.nft_mint_owner_ata;
    let token_program = ctx.accounts.token_program.to_account_info();
    let charger_pda = &mut ctx.accounts.charger_pda;

    require!(
        mint.key() == Pubkey::from_str(DEFAULT_TOKEN_MINT).unwrap(),
        DplError::InvalidMint
    );

    require!(amount > 0, DplError::InvalidAmount);
    require!(user_ata.amount >= amount as u64, DplError::InvalidAmount);

    transfer_checked(
        CpiContext::new(
            token_program.to_account_info(),
            TransferChecked {
                from: user_ata.to_account_info(),
                mint: mint.to_account_info(),
                to: operator_ata.to_account_info(),
                authority: user.to_account_info(),
            },
        ),
        amount * 0.3 as u64 * 10_u64.pow(mint.decimals.into()),
        mint.decimals,
    )?;

    transfer_checked(
        CpiContext::new(
            token_program.to_account_info(),
            TransferChecked {
                from: user_ata.to_account_info(),
                mint: mint.to_account_info(),
                to: nft_mint_owner_ata.to_account_info(),
                authority: user.to_account_info(),
            },
        ),
        amount * 0.7 as u64 * 10_u64.pow(mint.decimals.into()),
        mint.decimals,
    )?;

    charger_pda.all_time_revenue += amount;

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
    /// CHECK: charger unsafe acc
    pub charger: AccountInfo<'info>,
    #[account(
        seeds = [b"charger", charger.key().as_ref()],
        bump
    )]
    pub charger_pda: Account<'info, Charger>,
    pub mint: Account<'info, Mint>,
    pub nft_mint: Account<'info, Mint>,
    /// CHECK: nft_mint_owner unsafe acc
    pub nft_mint_owner: AccountInfo<'info>,
    #[account(
        token::mint = mint,
        token::authority = nft_mint_owner
    )]
    pub nft_mint_owner_ata: Account<'info, TokenAccount>,
    /// CHECK:
    pub operator: AccountInfo<'info>,
    #[account(
        token::mint = mint,
        token::authority = operator
    )]
    pub operator_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
