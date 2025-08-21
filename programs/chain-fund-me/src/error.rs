use anchor_lang::prelude::*;


#[error_code]
pub enum CrowdfundingError {
    #[msg("Not authorized as factory owner")]
    NotFactoryOwner,
    #[msg("Not authorized as moderator")]
    NotModerator,
    #[msg("Not campaign owner")]
    NotCampaignOwner,
    #[msg("Campaign funding period has not started")]
    FundingNotStarted,
    #[msg("Campaign funding period has ended")]
    FundingPeriodOver,
    #[msg("Campaign is still active")]
    CampaignStillActive,
    #[msg("Factory contract is paused")]
    FactoryPaused,
    #[msg("Campaign is paused")]
    CampaignPaused,
    #[msg("Points system is paused")]
    PointsPaused,
    #[msg("Token not in accepted list")]
    TokenNotAccepted,
    #[msg("Invalid funding amount")]
    InvalidAmount,
    #[msg("Invalid Tier")]
    InvalidTier,
    #[msg("Campaign funding not approved")]
    FundingNotApproved,
    #[msg("Campaign funding disapproved")]
    FundingDisapproved,
    #[msg("Withdrawal not approved")]
    WithdrawalNotApproved,
    #[msg("Withdrawal approval revoked")]
    WithdrawalRevoked,
    #[msg("Already owns this multiplier tier")]
    MultiplierAlreadyOwned,
    #[msg("Insufficient SOL for multiplier tier")]
    InsufficientSolForTier,
    #[msg("Platform fee must be between 1-20%")]
    InvalidPlatformFee,
    #[msg("Maximum 5 tokens allowed per campaign")]
    TooManyTokens,
    #[msg("Invalid campaign dates")]
    InvalidDates,
    #[msg("Campaign already approved")]
    AlreadyApproved,
    #[msg("Accounts provided do not match expected format")]
    InvalidAccounts,
    #[msg("Overflow error")]
    Overflow,
    #[msg("Insufficient funds for operation")]
    InsufficientFunds,


}