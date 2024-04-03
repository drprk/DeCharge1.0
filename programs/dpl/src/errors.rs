use anchor_lang::prelude::*;

#[error_code]
pub enum DplError {
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Invalid amount")]
    InvalidAmount,
}
