use anchor_lang::prelude::*;

#[error_code]
pub enum DplError {
    #[msg("Phone number hash must be 32 bytes")]
    PhoneNumberHash32Bytes,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Invalid amount")]
    InvalidAmount,
}
