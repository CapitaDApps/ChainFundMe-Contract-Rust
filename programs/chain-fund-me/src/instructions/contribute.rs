use crate::events::Deposited;
use crate::Campaign;
use crate::{Contribution, CrowdfundingError, Factory, Funder, Spender};
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::{Token, Transfer};

#[derive(Accounts)]
pub struct Contribute<'info> {
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
    ///CHECK:  This ensures fee wallet is a valid account
    #[account(mut)]
    pub fee_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub campaign_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub contributor_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_wallet_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

    #[account(
        init_if_needed,
        payer = contributor,
        space = 8 + Spender::INIT_SPACE,
        seeds = [b"spender", contributor.key().as_ref()],
        bump
    )]
    pub spender: Account<'info, Spender>,
}

pub fn process_contribute(ctx: Context<Contribute>, amount: u64, is_token: bool) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;
    let contribution = &mut ctx.accounts.contribution;
    let factory = &ctx.accounts.factory;

    require!(!campaign.is_paused, CrowdfundingError::CampaignPaused);
    require!(!campaign.ended, CrowdfundingError::FundingPeriodOver);
    require!(
        Clock::get()?.unix_timestamp >= campaign.start_time,
        CrowdfundingError::FundingNotStarted
    );
    require!(
        Clock::get()?.unix_timestamp < campaign.end_time,
        CrowdfundingError::FundingPeriodOver
    );
    require!(amount > 0, CrowdfundingError::InvalidAmount);
    require!(!factory.is_paused, CrowdfundingError::FactoryPaused);
    if factory.limits_enabled {
        require!(
            campaign.funding_approved,
            CrowdfundingError::FundingNotApproved
        );
        require!(
            !campaign.funding_disapproved,
            CrowdfundingError::FundingDisapproved
        );
    }

    let fee = (amount as u128 * factory.platform_fee as u128 / 100) as u64;
    let net_amount = amount - fee;

    if is_token {
        // Transfer tokens (SPL equivalent of ERC20)
        let cpi_accounts = Transfer {
            from: ctx.accounts.contributor_token.to_account_info(),
            to: ctx.accounts.campaign_token.to_account_info(),
            authority: ctx.accounts.contributor.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(
            CpiContext::new(cpi_program.clone(), cpi_accounts),
            net_amount,
        )?;

        // Transfer fee to fee wallet
        let cpi_accounts_fee = Transfer {
            from: ctx.accounts.contributor_token.to_account_info(),
            to: ctx.accounts.fee_wallet_token.to_account_info(),
            authority: ctx.accounts.contributor.to_account_info(),
        };
        token::transfer(CpiContext::new(cpi_program.clone(), cpi_accounts_fee), fee)?;
    } else {
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

        **campaign.to_account_info().try_borrow_mut_lamports()? -= fee;
        **ctx
            .accounts
            .fee_wallet
            .to_account_info()
            .try_borrow_mut_lamports()? += fee;
    }

    contribution.contributor = ctx.accounts.contributor.key();
    contribution.campaign = campaign.key();
    if is_token {
        contribution.token_amount += amount;
    } else {
        contribution.sol_amount += amount;
    }

    // Update campaign funders
    campaign.funders_count += 1;
    campaign.funders.push(Funder {
        funder_address: ctx.accounts.contributor.key(),
        token_mint: if is_token {
            ctx.accounts.campaign_token.mint
        } else {
            Pubkey::default()
        },
        amount,
    });

    // Mint points (simplified, assumes SOL = 1 USD for demo)
    let points = amount; // Replace with oracle-based conversion
    let multiplier = ctx.accounts.spender.multiplier;
    ctx.accounts.spender.points_earned += points * multiplier as u64;

    emit!(Deposited {
        funder: ctx.accounts.contributor.key(),
        token: if is_token {
            ctx.accounts.campaign_token.mint
        } else {
            Pubkey::default()
        },
        amount,
    });

    Ok(())
}
