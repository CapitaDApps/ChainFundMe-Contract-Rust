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

    #[max_len(5)]
    pub moderators: Vec<Moderators>,
    #[max_len(5)]
    pub other_accepted_tokens: Vec<AcceptedToken>,

    // pub verified_creators: Vec<(Pubkey, bool)>,
}
