use anchor_lang::prelude::*;


#[account]
pub struct Campaign {
    pub owner: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub metadata_uri: String,
    pub stablecoin_mint: Pubkey,
    pub other_acceptable_tokens: Vec<Pubkey>,
    pub factory: Pubkey,
    pub is_paused: bool,
    pub is_withdraw_approved: bool,
    pub is_withdrawal_points_minted: bool,
    pub withdrawal_approval_revoked: bool,
    pub funding_approved: bool,
    pub funding_disapproved: bool,
    pub ended: bool,
    pub funders_count: u64,
    pub funders: Vec<Funder>,
}