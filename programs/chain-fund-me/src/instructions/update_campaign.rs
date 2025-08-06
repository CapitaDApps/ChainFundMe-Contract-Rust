use anchor_lang::prelude::*;

use crate::Campaign;
use crate::CrowdfundingError;

#[derive(Accounts)]
pub struct UpdateCampaignTime<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign
    >,
    pub owner: Signer<'info>,
}



pub fn process_update_campaign_time(ctx: Context<UpdateCampaignTime>, start_time: Option<i64>, end_time: Option<i64>) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        require!(campaign.owner == ctx.accounts.owner.key(), CrowdfundingError::NotCampaignOwner);
        
        let current_time = Clock::get()?.unix_timestamp;
        
        if let Some(new_start_time) = start_time {
            require!(
                current_time < campaign.start_time || current_time > campaign.end_time,
                CrowdfundingError::CampaignStillActive
            );
            require!(!campaign.ended, CrowdfundingError::FundingPeriodOver);
            campaign.start_time = new_start_time;
        }
        
        if let Some(new_end_time) = end_time {
            require!(!campaign.ended, CrowdfundingError::FundingPeriodOver);
            campaign.end_time = new_end_time;
        }
        
        Ok(())
    }
