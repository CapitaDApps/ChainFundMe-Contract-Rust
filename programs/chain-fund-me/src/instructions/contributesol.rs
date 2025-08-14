use crate::{events::Deposited, Campaign, Contribution, CrowdfundingError, Factory, Funder, Spender};
use anchor_lang::prelude::*;
use anchor_lang::system_program;

#[derive(Accounts)]
pub struct ContributeSol<'info> {
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
    /// CHECK: just a lamports receiver
    #[account(mut)]
    pub fee_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub spender: Account<'info, Spender>,
}

pub fn process_contribute_sol(ctx: Context<ContributeSol>, amount: u64) -> Result<()> {
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

    // Transfer SOL to campaign
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.contributor.to_account_info(),
                to: campaign.to_account_info(),
            },
        ),
        amount,
    )?;

    // Deduct fee from campaign to fee_wallet
    **campaign.to_account_info().try_borrow_mut_lamports()? -= fee;
    **ctx.accounts.fee_wallet.to_account_info().try_borrow_mut_lamports()? += fee;

    // Record contribution
    contribution.contributor = ctx.accounts.contributor.key();
    contribution.campaign = campaign.key();
    contribution.sol_amount += amount;

    campaign.funders_count += 1;
    campaign.funders.push(Funder {
        funder_address: ctx.accounts.contributor.key(),
        token_mint: Pubkey::default(),
        amount,
    });

    let points = amount;
    let multiplier = ctx.accounts.spender.multiplier;
    ctx.accounts.spender.points_earned += points * multiplier as u64;

    emit!(Deposited {
        funder: ctx.accounts.contributor.key(),
        token: Pubkey::default(),
        amount,
    });

    Ok(())
}
