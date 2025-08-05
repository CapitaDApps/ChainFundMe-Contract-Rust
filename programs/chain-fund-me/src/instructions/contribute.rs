use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(
        init_if_needed,
        payer = contributor,
        space = 8 + 32 + 32 + 8 + 8, // Approx space for Contribution
        seeds = [b"contribution", contributor.key().as_ref(), campaign.key().as_ref()],
        bump
    )]
    pub contribution: Account<'info, Contribution>,
    #[account(mut)]
    pub contributor: Signer<'info>,
    #[account(mut)]
    pub factory: Account<'info, Factory>,
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
    #[account(mut)]
    pub spender: Account<'info, Spender>,
}

pub fn process_contribute(
        ctx: Context<Contribute>,
        amount: u64,
        is_token: bool,
    ) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let contribution = &mut ctx.accounts.contribution;
        let factory = &ctx.accounts.factory;

        require!(!campaign.is_paused, ErrorCode::FundingPaused);
        require!(!campaign.ended, ErrorCode::FundingPeriodOver);
        require!(
            Clock::get()?.unix_timestamp >= campaign.start_time,
            FundingPeriodNotStarted
        );
        require!(
            Clock::get()?.unix_timestamp < campaign.end_time,
            CrowdfundingError::FundingPeriodOver
        );
        require!(amount > 0, InvalidAmount);
        require!(!factory.is_paused, FactoryPaused);
        if factory.limits_enabled {
            require!(campaign.funding_approved, FundingNotApproved);
            require!(!campaign.funding_disapproved, FundingDisapproved);
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
            token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

            // Transfer fee to fee wallet
            let cpi_accounts_fee = Transfer {
                from: ctx.accounts.contributor_token.to_account_info(),
                to: ctx.accounts.fee_wallet_token.to_account_info(),
                authority: ctx.accounts.contributor.to_account_info(),
            };
            token::transfer(CpiContext::new(cpi_program, cpi_accounts_fee), fee)?;
        } else {
            // Transfer SOL
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.contributor.to_account_info(),
                        to: ctx.accounts.campaign.to_account_info(),
                    },
                ),
                net_amount,
            )?;

            // Transfer fee to fee wallet
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.contributor.to_account_info(),
                        to: ctx.accounts.fee_wallet.to_account_info(),
                    },
                ),
                fee,
            )?;
        }

        // Update contribution
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
            token_mint: if is_token { ctx.accounts.campaign_token.mint } else { Pubkey::default() },
            amount,
        });

        // Mint points (simplified, assumes SOL = 1 USD for demo)
        let points = amount; // Replace with oracle-based conversion
        let multiplier = ctx.accounts.spender.multiplier;
        ctx.accounts.spender.points_earned += points * multiplier as u64;

        emit!(Deposited {
            funder: ctx.accounts.contributor.key(),
            token: if is_token { ctx.accounts.campaign_token.mint } else { Pubkey::default() },
            amount,
        });

        Ok(())
    }