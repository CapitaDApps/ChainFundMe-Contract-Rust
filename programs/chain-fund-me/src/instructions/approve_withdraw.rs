use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ApproveWithdraw<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    pub moderator: Signer<'info>,
    #[account(has_one = moderators @ NotModerator)]
    pub factory: Account<'info, Factory>,
}

pub fn approve_withdraw(ctx: Context<ApproveWithdraw>) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        require!(!campaign.is_withdraw_approved, AlreadyApproved);
        require!(
            Clock::get()?.unix_timestamp >= campaign.start_time,
            FundingPeriodNotStarted
        );
        require!(
            Clock::get()?.unix_timestamp >= campaign.end_time || campaign.ended,
            FundingStillActive
        );

        campaign.is_withdraw_approved = true;
        campaign.ended = true;
        emit!(WithdrawApproved {});
        Ok(())
    }
