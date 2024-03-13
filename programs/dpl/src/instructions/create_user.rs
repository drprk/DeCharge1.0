use anchor_lang::prelude::*;

use crate::errors::DplError;
use crate::state::user::User;

pub fn create_user_ix(ctx: Context<CreateUser>, phone_number_hash: String) -> Result<()> {
    let user_pda = &mut ctx.accounts.user_pda;

    let clock: Clock = Clock::get()?;

    require!(
        phone_number_hash.len() == 32,
        DplError::PhoneNumberHash32Bytes
    );

    user_pda.pubkey = *ctx.accounts.user.key;
    user_pda.created_at = clock.unix_timestamp;
    user_pda.phone_number_hash = phone_number_hash;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        seeds = [b"user", user.key().as_ref()],
        bump,
        payer = user,
        space = User::LEN,
    )]
    pub user_pda: Account<'info, User>,
    pub system_program: Program<'info, System>,
}
