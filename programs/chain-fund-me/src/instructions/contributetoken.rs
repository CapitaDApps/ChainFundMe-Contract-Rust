use crate::{events::Deposited, Campaign, Contribution, CrowdfundingError, Factory, Funder, Spender};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Token, Transfer};

#[derive(Accounts)]
pub struct ContributeToken<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(
        init_if_needed,
        payer = contributor,
        space = 8 + Contribution::INIT_SPACE,
        seeds = [b"contribution", contributor.key().as_ref(), campaign.key().as_ref()],
        bump
    )]
    pub contribution: Account<'info, Contribution>,
    #[account(mut)]
    pub contributor: Signer<'info>,
    #[account(mut)]
    pub factory: Account<'info, Factory>,
    #[account(mut)]
    pub campaign_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub contributor_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_wallet_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub spender: Account<'info, Spender>,
    pub system_program: Program<'info, System>
}

pub fn process_contribute_token(ctx: Context<ContributeToken>, amount: u64) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;
    let contribution = &mut ctx.accounts.contribution;
    let factory = &ctx.accounts.factory;

    require!(!campaign.is_paused, CrowdfundingError::CampaignPaused);
    require!(!campaign.ended, CrowdfundingError::FundingPeriodOver);
    require!(Clock::get()?.unix_timestamp >= campaign.start_time, CrowdfundingError::FundingNotStarted);
    require!(Clock::get()?.unix_timestamp < campaign.end_time, CrowdfundingError::FundingPeriodOver);
    require!(amount > 0, CrowdfundingError::InvalidAmount);
    require!(!factory.is_paused, CrowdfundingError::FactoryPaused);

    if factory.limits_enabled {
        require!(campaign.funding_approved, CrowdfundingError::FundingNotApproved);
        require!(!campaign.funding_disapproved, CrowdfundingError::FundingDisapproved);
    }

    let fee = (amount as u128 * factory.platform_fee as u128 / 100) as u64;
    let net_amount = amount - fee;

    // Transfer tokens to campaign
    let cpi_accounts = Transfer {
        from: ctx.accounts.contributor_token.to_account_info(),
        to: ctx.accounts.campaign_token.to_account_info(),
        authority: ctx.accounts.contributor.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program.clone(), cpi_accounts), net_amount)?;

    // Transfer fee tokens
    let cpi_accounts_fee = Transfer {
        from: ctx.accounts.contributor_token.to_account_info(),
        to: ctx.accounts.fee_wallet_token.to_account_info(),
        authority: ctx.accounts.contributor.to_account_info(),
    };
    token::transfer(CpiContext::new(cpi_program, cpi_accounts_fee), fee)?;

    // Record contribution
    contribution.contributor = ctx.accounts.contributor.key();
    contribution.campaign = campaign.key();
    contribution.token_amount += amount;

    campaign.funders_count += 1;
    campaign.funders.push(Funder {
        funder_address: ctx.accounts.contributor.key(),
        token_mint: ctx.accounts.campaign_token.mint,
        amount,
    });

    let points = amount;
    let multiplier = ctx.accounts.spender.multiplier;
    ctx.accounts.spender.points_earned += points * multiplier as u64;

    emit!(Deposited {
        funder: ctx.accounts.contributor.key(),
        token: ctx.accounts.campaign_token.mint,
        amount,
    });

    Ok(())
}
