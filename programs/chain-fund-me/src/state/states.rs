use anchor_lang::prelude::*;


#[account]
pub struct Contribution {
    pub contributor: Pubkey,
    pub campaign: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
}

#[account]

pub struct Points {
    pub owner: Pubkey,
    pub is_paused: bool,
    pub multiplier_tiers: Vec<(MultiplierTier, MultiplierInfo)>,
}

#[account]
pub struct Spender {
    pub owner: Pubkey,
    pub multiplier_tier: MultiplierTier,
    pub multiplier: u32,
    pub points_earned: u64,
}

#[account]
pub struct Funder {
    pub funder_address: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
}
#[derive(
    Clone,
    Debug,
    AnchorSerialize,
    AnchorDeserialize,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,  // ✅ Add this
    Ord          // ✅ And this
)]
pub enum MultiplierTier {
    Base,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}
#[account]
pub struct MultiplierInfo {
    pub price: u64, // SOL lamports
    pub multiplier: u32,
}

#[account]
pub struct PointsConfig {
    pub owner: Pubkey,
    pub factory: Pubkey,
    pub base_points: u64,
    pub paused: bool,
}