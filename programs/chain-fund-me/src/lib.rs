pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod events;
pub use events::*;
use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use error::*;

declare_id!("B2FS9dP7KUp5ptuMZCnc1JMDFCRNbqxrWRisXSwHk6rs");

#[program]
pub mod chain_fund_me {

    use super::*;

    pub fn initialize_factory(ctx: Context<InitializeFactory>, platform_fee:u8, stablecoin_mint: Pubkey, fee_wallet: Pubkey) -> Result<()> {
        process_initialize_factory(ctx, platform_fee, stablecoin_mint, fee_wallet)
    }
    
    pub fn create_campaign(
        ctx: Context<CreateCampaign>,

        start_time: i64,
        end_time: i64,
        metadata_uri: String,
        other_token_mints: Vec<Pubkey>,
    ) -> Result<()>{
        process_create_campaign(ctx, start_time, end_time, metadata_uri, other_token_mints)
    }


    pub fn contribute(ctx:Context<Contribute>, amount: u64, is_token:bool) -> Result<()> {
        process_contribute(ctx, amount, is_token)
    }

    pub fn update_campaign_time(
        ctx: Context<UpdateCampaignTime>,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<()> {
        process_update_campaign_time(ctx, start_time, end_time)
    }

    pub fn pause_campaign(ctx: Context<PauseCampaign>, paused: bool) -> Result<()> {
        process_pause_campaign(ctx, paused)
    }   


    pub fn withdraw(ctx: Context<Withdraw>, is_token:bool)-> Result<()>{
        process_withdraw(ctx, is_token)
    }


}

