use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub pubkey: Pubkey,
    pub created_at: i64,
    pub phone_number_hash: String,
}

impl User {
    pub const LEN: usize = 8 + 32 + 8 + (4 + (4 * 128));
}
