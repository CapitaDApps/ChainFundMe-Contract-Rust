pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
use anchor_lang::prelude::*;
pub use events::*;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("Ej3tE117rHTNG6rJwrikXB45XXYJefiBUJMmv5GqmeNQ");

#[program]
pub mod chain_fund_me {

    use super::*;

    pub fn initialize_factory(
        ctx: Context<InitializeFactory>,
        factory_id: u64,
        platform_fee: u8,
        stablecoin_mint: Pubkey,
        fee_wallet: Pubkey,
        other_accepted_tokens: Vec<AcceptedToken>,
    ) -> Result<()> {
        process_initialize_factory(
            ctx,
            factory_id,
            platform_fee,
            stablecoin_mint,
            fee_wallet,
            other_accepted_tokens,
        )
    }

    pub fn create_campaign(
        ctx: Context<CreateCampaign>,

        start_time: i64,
        end_time: i64,
        metadata_uri: String,
        other_token_mints: Vec<Pubkey>,
    ) -> Result<()> {
        process_create_campaign(ctx, start_time, end_time, metadata_uri, other_token_mints)
    }

    pub fn contribute_sol(ctx: Context<ContributeSol>, amount: u64) -> Result<()> {
        process_contribute_sol(ctx, amount)
    }

    pub fn contribute_token(ctx: Context<ContributeToken>, amount: u64) -> Result<()> {
        process_contribute_token(ctx, amount)
    }

    pub fn update_campaign_time(
        ctx: Context<UpdateCampaignTime>,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<()> {
        process_update_campaign_time(ctx, start_time, end_time)
    }

    pub fn pause_campaign(
        ctx: Context<PauseCampaign>,
        paused: bool,
        campaign_id: u64,
    ) -> Result<()> {
        process_pause_campaign(ctx, paused, campaign_id)
    }

    pub fn end_campaign(ctx: Context<EndCampaign>)-> Result<()>{
        process_end_campaign(ctx)
    }

    pub fn withdraw<'info>(ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>) -> Result<()> {
        process_withdraw(ctx)
    }
}
