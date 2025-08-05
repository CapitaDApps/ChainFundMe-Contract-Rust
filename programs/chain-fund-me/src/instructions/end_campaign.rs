use anchor_lang::prelude::*;

pub fn end_campaign(ctx: Context<EndCampaign>) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        require!(campaign.owner == ctx.accounts.owner.key(), CrowdfundingError::NotCampaignOwner);
        require!(Clock::get()?.unix_timestamp >= campaign.start_time, CrowdfundingError::FundingNotStarted);
        require!(Clock::get()?.unix_timestamp <= campaign.end_time && !campaign.ended, CrowdfundingError::FundingPeriodOver);
        
        campaign.ended = true;
        campaign.end_time = Clock::get()?.unix_timestamp;
        
        Ok(())
    }