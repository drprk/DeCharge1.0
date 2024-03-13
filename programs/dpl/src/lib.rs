use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::create_charger::*;
use instructions::create_user::*;

declare_id!("6NumYGQrACaBTDiNwHyvmWihxejgGHDMJrEQYD7sTBN9");

#[program]
pub mod dpl {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, phone_number_hash: String) -> Result<()> {
        create_user_ix(ctx, phone_number_hash)
    }

    pub fn create_charger(ctx: Context<CreateCharger>) -> Result<()> {
        create_charger_ix(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
