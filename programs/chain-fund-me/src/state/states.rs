use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Contribution {
    pub contributor: Pubkey,
    pub campaign: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct AcceptedToken {
    pub mint: Pubkey,
    pub allowed: bool,
}

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
// pub struct VerifiedCreators {
//     pub creator: Pubkey,
//     pub allowed: bool
// }

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
// pub struct Campaigns {
//     pub campaign: Pubkey,
//     pub campaign_id: u64,
// }


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Moderators {
    pub moderator: Pubkey,
    pub allowed: bool
}



#[account]
pub struct Points {
    pub owner: Pubkey,
    pub is_paused: bool,
    pub multiplier_tiers: Vec<(MultiplierTier, MultiplierInfo)>,
}

#[account]
#[derive(InitSpace)]
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
    InitSpace,
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
