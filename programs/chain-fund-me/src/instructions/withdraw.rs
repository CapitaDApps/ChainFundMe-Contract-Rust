use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;
use anchor_spl::token::{self, Token, TokenAccount};

use crate::{Campaign, CrowdfundingError, Factory, Spender, BASE_POINTS};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut, 
        has_one = owner,
        seeds = [b"campaign", owner.key().as_ref(), &campaign.start_time.to_le_bytes()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,
    pub factory: Account<'info, Factory>,
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub campaign_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub spender: Account<'info, Spender>,

    pub system_program: Program<'info, System>,
}

pub fn process_withdraw(ctx: Context<Withdraw>, is_token: bool) -> Result<()> {
    let owner_key = ctx.accounts.owner.key();
    let start_time_bytes = ctx.accounts.campaign.start_time.to_le_bytes();

    let campaign = &mut ctx.accounts.campaign;
    let factory = &ctx.accounts.factory;

    require!(!campaign.is_paused, CrowdfundingError::CampaignPaused);
    if factory.limits_enabled {
        require!(
            campaign.is_withdraw_approved,
            CrowdfundingError::WithdrawalNotApproved
        );
        require!(
            !campaign.withdrawal_approval_revoked,
            CrowdfundingError::WithdrawalNotApproved
        );
    }

    if is_token {
        let amount = ctx.accounts.campaign_token.amount;

        let cpi_accounts = Transfer {
            from: ctx.accounts.campaign_token.to_account_info(),
            to: ctx.accounts.owner_token.to_account_info(),
            authority: campaign.to_account_info(),
        };

        let seeds = &[
            b"campaign",
            owner_key.as_ref(),
            &start_time_bytes,
            &[campaign.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(
            CpiContext::new_with_signer(cpi_program, cpi_accounts, signer),
            amount,
        )?;
    } else {
        let balance = campaign.to_account_info().lamports();
        **campaign.to_account_info().try_borrow_mut_lamports()? -= balance;
        **ctx
            .accounts
            .owner
            .to_account_info()
            .try_borrow_mut_lamports()? += balance;
    }

    if !campaign.is_withdrawal_points_minted {
        ctx.accounts.spender.points_earned += BASE_POINTS;
        campaign.is_withdrawal_points_minted = true;
    }

    Ok(())
}
