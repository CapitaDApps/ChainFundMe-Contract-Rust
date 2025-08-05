#[account]
pub struct Factory {
    pub owner: Pubkey,
    pub platform_fee: u8,
    pub stablecoin_mint: Pubkey,
    pub fee_wallet: Pubkey,
    pub deployed_campaigns_count: u64,
    pub limits_enabled: bool,
    pub is_paused: bool,
    pub campaigns: HashMap<Pubkey, u64>,
    pub moderators: HashMap<Pubkey, bool>,
    pub other_accepted_tokens: HashMap<Pubkey, bool>,
    pub verified_creators: HashMap<Pubkey, bool>,
}