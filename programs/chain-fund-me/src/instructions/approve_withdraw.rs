use crate::events::WithdrawApproved;
use crate::{Campaign, CrowdfundingError, Factory};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ApproveWithdraw<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    pub moderator: Signer<'info>,

    pub factory: Account<'info, Factory>,
}
pub fn approve_withdraw(ctx: Context<ApproveWithdraw>) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;
    let factory = &ctx.accounts.factory;
    let signer = ctx.accounts.moderator.key();
    let is_moderator = factory.moderators.iter().any(|m| m.moderator == signer && m.allowed);
    require!(is_moderator, CrowdfundingError::NotModerator);

    require!(
        !campaign.is_withdraw_approved,
        CrowdfundingError::AlreadyApproved
    );
    require!(
        Clock::get()?.unix_timestamp >= campaign.start_time,
        CrowdfundingError::FundingNotStarted
    );
    require!(
        Clock::get()?.unix_timestamp >= campaign.end_time || campaign.ended,
        CrowdfundingError::CampaignStillActive
    );

    campaign.is_withdraw_approved = true;
    campaign.ended = true;

    emit!(WithdrawApproved {});
    Ok(())
}
