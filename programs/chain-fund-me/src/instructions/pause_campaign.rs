use anchor_lang::prelude::*;  
use crate::CrowdfundingError;
use crate::state::Campaign;

#[derive(Accounts)]
pub struct PauseCampaign<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    pub factory: Account<'info, FundingFactory>,
    pub moderator: Signer<'info>,
}


  pub fn pause_campaign(ctx: Context<PauseCampaign>, paused: bool) -> Result<()> {
        let factory = &ctx.accounts.factory;
        require!(factory.moderators.contains(&ctx.accounts.moderator.key()), CrowdfundingError::NotModerator);
        
        let campaign = &mut ctx.accounts.campaign;
        campaign.is_paused = false;
        
        Ok(())
    }