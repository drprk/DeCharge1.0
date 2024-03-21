use crate::state::charger::Charger;

use anchor_lang::prelude::*;
use anchor_spl::token::Token;

pub fn create_charger_ix(ctx: Context<CreateCharger>) -> Result<()> {
    let charger_pda = &mut ctx.accounts.charger_pda;
    let nft_mint = &ctx.accounts.nft_mint;
    let payer = &ctx.accounts.payer.key();

    msg!("nft mint owner {}", &nft_mint.owner);
    msg!("payer {}", payer);
    msg!("charger {}", &ctx.accounts.charger.key);
    msg!("charger pda {}", &charger_pda.key());

    // require!(&nft_mint.owner.eq(payer), DplError::InvalidMint);

    let clock: Clock = Clock::get()?;

    charger_pda.pubkey = *ctx.accounts.charger.key;
    charger_pda.operator = *ctx.accounts.operator.key;
    charger_pda.created_at = clock.unix_timestamp;
    charger_pda.nft_mint = nft_mint.key();
    charger_pda.all_time_revenue = 0;

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
    /// CHECK: operator unsafe acc
    pub operator: AccountInfo<'info>,

    /// CHECK: nft mint address
    pub nft_mint: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
