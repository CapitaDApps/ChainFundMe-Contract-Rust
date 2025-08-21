use crate::events::ChainFundMeCreated;
use crate::{Campaign, CrowdfundingError, Factory, Spender};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(start_time: i64, end_time: i64, metadata_uri: String, other_token_mints: Vec<Pubkey>)]
pub struct CreateCampaign<'info> {
    #[account(mut)]
    pub factory: Account<'info, Factory>,

    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 8 + 8 + 4 + metadata_uri.len() + 32 + 32 + 4 + other_token_mints.len() * 32 + 32 + 1 + 1 + 1 + 1 + 1 + 8 + 4 + 1024,
        seeds = [b"campaign", creator.key().as_ref(), start_time.to_le_bytes().as_ref()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(
        init,
        payer = creator,
        space = 8 + Spender::INIT_SPACE,
        seeds = [b"spender", contributor.key().as_ref()],
        bump
    )]
    pub spender: Account<'info, Spender>,

    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn process_create_campaign(
    ctx: Context<CreateCampaign>,
    start_time: i64,
    end_time: i64,
    metadata_uri: String,
    other_token_mints: Vec<Pubkey>,
) -> Result<()> {
    let factory = &mut ctx.accounts.factory;
    let campaign = &mut ctx.accounts.campaign;

    require!(!factory.is_paused, CrowdfundingError::FactoryPaused);
    require!(start_time < end_time, CrowdfundingError::InvalidDates);
    require!(
        start_time > Clock::get()?.unix_timestamp,
        CrowdfundingError::InvalidDates
    );
    require!(
        other_token_mints.len() <= 5,
        CrowdfundingError::TooManyTokens
    );

    campaign.owner = ctx.accounts.creator.key();
    campaign.start_time = start_time;
    campaign.end_time = end_time;
    campaign.metadata_uri = metadata_uri;
    campaign.stablecoin_mint = factory.stablecoin_mint;
    campaign.other_acceptable_tokens = other_token_mints;
    campaign.factory = factory.key();
    campaign.funders_count = 0;
    campaign.is_paused = false;
    campaign.ended = false;
    campaign.bump = ctx.bumps.campaign;
    let campaign_id = factory.deployed_campaigns_count;
    factory.campaigns.push((campaign.key(), campaign_id));

    emit!(ChainFundMeCreated {
        creator: ctx.accounts.creator.key(),
        campaign: campaign.key(),
    });
    

    Ok(())
}
