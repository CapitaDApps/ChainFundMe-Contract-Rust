use anchor_lang::prelude::*;

#[account]
pub struct Factory {
    pub factory_id: u64,
    pub owner: Pubkey,
    pub platform_fee: u8,
    pub stablecoin_mint: Pubkey,
    pub fee_wallet: Pubkey,
    pub deployed_campaigns_count: u64,
    pub limits_enabled: bool,
    pub is_paused: bool,
    pub campaigns: Vec<(Pubkey, u64)>,
    pub moderators: Vec<(Pubkey, bool)>,
    pub other_accepted_tokens: Vec<(Pubkey, bool)>,
    pub verified_creators: Vec<(Pubkey, bool)>,
}
