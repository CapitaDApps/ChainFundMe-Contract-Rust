use crate::state::{Campaign, Factory};
use crate::CrowdfundingError;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PauseCampaign<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    pub factory: Account<'info, Factory>,
    pub moderator: Signer<'info>,
}

pub fn process_pause_campaign(ctx: Context<PauseCampaign>, paused: bool, campaign_id: u64) -> Result<()> {
    let factory = &ctx.accounts.factory;

    let is_moderator = factory
        .moderators
        .iter()
        .any(|m| m.moderator == ctx.accounts.moderator.key());

    require!(is_moderator, CrowdfundingError::NotModerator);

    let campaign = &mut ctx.accounts.campaign;
    campaign.is_paused = paused;

    Ok(())
}
