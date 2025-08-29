use anchor_lang::prelude::*;

use crate::{AcceptedToken, Moderators};

#[account]
#[derive(InitSpace)]
pub struct Factory {
    pub factory_id: u64,
    pub owner: Pubkey,
    pub platform_fee: u8,
    pub stablecoin_mint: Pubkey,
    pub fee_wallet: Pubkey,
    pub deployed_campaigns_count: u64,
    pub limits_enabled: bool,
    pub is_paused: bool,
    // #[max_len(100)]
    // pub campaigns: Vec<Campaigns>,
    #[max_len(5)]
    pub moderators: Vec<Moderators>,
    #[max_len(5)]
    pub other_accepted_tokens: Vec<AcceptedToken>,
    // #[max_len(10)]
    // pub verified_creators: Vec<VerifiedCreators>,
}


impl Factory {
    // Calculate space manually
    pub const fn space() -> usize {
        8 + // discriminator
        8 + // factory_id
        32 + // owner
        1 + // platform_fee
        32 + // stablecoin_mint
        32 + // fee_wallet
        8 + // deployed_campaigns_count
        1 + // limits_enabled
        1 + // is_paused
        4 + (5 * (32 + 1)) + // moderators vec (assuming Moderators is 33 bytes)
        4 + (5 * (32 + 1))   // other_accepted_tokens vec (33 bytes each)
    }
}