use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Charger {
    pub pubkey: Pubkey,
    pub created_at: i64,
    pub nft_mint: Pubkey,
    pub operator: Pubkey,
    pub all_time_revenue: u64,
}

impl Charger {
    pub const LEN: usize = 8 + 32 + 8 + 32 + 32 + 8;
}
