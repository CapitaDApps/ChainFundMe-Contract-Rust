use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::{Points, Spender, CrowdfundingError};
use crate::events::PurchasedMultiplier;
use crate::MultiplierTier;


#[derive(Accounts)]
pub struct PurchaseMultiplier<'info> {
    #[account(mut)]
    pub points: Account<'info, Points>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 32 + 1 + 4 + 8, 
        seeds = [b"spender", payer.key().as_ref()],
        bump
    )]
    pub spender: Account<'info, Spender>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    ///CHECK - Checks if this is a valid wallet
    pub fee_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}



    pub fn purchase_multiplier(
        ctx: Context<PurchaseMultiplier>,
        tier: MultiplierTier,
        amount: u64,
    ) -> Result<()> {
        let spender = &mut ctx.accounts.spender;
        require!(!ctx.accounts.points.is_paused, CrowdfundingError::PointsPaused);
        require!(spender.multiplier_tier != tier, CrowdfundingError::MultiplierAlreadyOwned);

        let tier_info = ctx.accounts.points
    .multiplier_tiers
    .iter()
    .find(|(t, _)| *t == tier)
    .map(|(_, info)| info)
    .ok_or(error!(CrowdfundingError::InvalidTier))?;
        // Hardcoded SOL prices (replace with oracle)
        let required_sol = tier_info.price; // Example: 0.01 SOL for Bronze
        require!(amount >= required_sol, CrowdfundingError::InsufficientSolForTier);

        // Refund excess SOL
        if amount > required_sol {
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.payer.to_account_info(),
                        to: ctx.accounts.payer.to_account_info(),
                    },
                ),
                amount - required_sol,
            )?;
        }

        // Transfer SOL to fee wallet
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.fee_wallet.to_account_info(),
                },
            ),
            required_sol,
        )?;

   
spender.multiplier_tier = tier.clone();
spender.multiplier = tier_info.multiplier;
spender.points_earned += amount;


emit!(PurchasedMultiplier {
    spender: ctx.accounts.payer.key(),
    tier,
    price_paid: amount,
    multiplier: tier_info.multiplier,
});

        Ok(())
    }