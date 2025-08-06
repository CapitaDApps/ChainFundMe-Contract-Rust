use anchor_lang::prelude::*;

use crate::MultiplierTier;
#[event]
pub struct ChainFundMeCreated {
    pub creator: Pubkey,
    pub campaign: Pubkey,
}

#[event]
pub struct Deposited {
    pub funder: Pubkey,
    pub token: Pubkey,
    pub amount: u64,
}

#[event]
pub struct WithdrawApproved {}


#[event]
pub struct PurchaseMultiplier {
    pub spender: Pubkey,
    pub tier: MultiplierTier,
    pub price_paid: u64,
    pub multiplier: u32,
}