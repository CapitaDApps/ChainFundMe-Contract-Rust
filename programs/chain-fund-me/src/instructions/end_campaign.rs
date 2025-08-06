use anchor_lang::prelude::*;

use crate::Campaign;
use crate::CrowdfundingError;

#[derive(Accounts)]
pub struct EndCampaign<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    pub owner: Signer<'info>,
}


pub fn end_campaign(ctx: Context<EndCampaign>) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        require!(campaign.owner == ctx.accounts.owner.key(), CrowdfundingError::NotCampaignOwner);
        require!(Clock::get()?.unix_timestamp >= campaign.start_time, CrowdfundingError::FundingNotStarted);
        require!(Clock::get()?.unix_timestamp <= campaign.end_time && !campaign.ended, CrowdfundingError::FundingPeriodOver);
        
        campaign.ended = true;
        campaign.end_time = Clock::get()?.unix_timestamp;
        
        Ok(())
    }