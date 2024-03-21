use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::charger_session::*;
use instructions::create_charger::*;
use instructions::create_user::*;

declare_id!("MfQ5MtGrou6TxQuBSGAFiQMuPTUWe7Y7kscbswUw31c");

#[program]
pub mod dpl {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, phone_number_hash: String) -> Result<()> {
        create_user_ix(ctx, phone_number_hash)
    }

    pub fn create_charger(ctx: Context<CreateCharger>) -> Result<()> {
        create_charger_ix(ctx)
    }

    pub fn charger_session(ctx: Context<ChargerSession>, amount: u64) -> Result<()> {
        charger_session_ix(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
