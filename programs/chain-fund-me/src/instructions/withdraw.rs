use anchor_lang::prelude::*;
use crate::state::campaign::Campaign;

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut, constraint = campaign.owner == owner.key() @ NotOwner)]
    pub owner: Signer<'info>,
    pub factory: Account<'info, Factory>,
    #[account(mut)]
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub spender: Account<'info, Spender>,
}


pub fn withdraw(ctx: Context<WithdrawSol>) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let factory = &ctx.accounts.factory;

        require!(!campaign.is_paused, FundingPaused);
        if factory.limits_enabled {
            require!(campaign.is_withdraw_approved, NotApproved);
            require!(!campaign.withdrawal_approval_revoked, NotApproved);
        }

        let balance = campaign.to_account_info().lamports();
        **campaign.to_account_info().try_borrow_mut_lamports()? -= balance;
        **ctx.accounts.owner.to_account_info().try_borrow_mut_lamports()? += balance;

        // Mint points if not already minted
        if !campaign.is_withdrawal_points_minted {
            ctx.accounts.spender.points_earned += BASE_POINTS;
            campaign.is_withdrawal_points_minted = true;
        }

        emit!(WithdrawnSol {
            owner: ctx.accounts.owner.key(),
            amount: balance,
        });

        Ok(())
    }