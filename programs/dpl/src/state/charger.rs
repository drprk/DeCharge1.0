use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Charger {
    pub pubkey: Pubkey,
    pub created_at: i64,
    pub nft_mint: Pubkey,
    pub all_time_revenue: u64,
    pub revenue_to_pay: u64,
}

impl Charger {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8;
}
