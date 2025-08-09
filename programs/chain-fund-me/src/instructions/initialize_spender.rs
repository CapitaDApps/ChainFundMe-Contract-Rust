use crate::Spender;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeSpender<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    #[account(
        init,
        payer = contributor,
        space = 8 + Spender::INIT_SPACE,
        seeds = [b"spender", contributor.key().as_ref()],
        bump
    )]
    pub spender: Account<'info, Spender>,

    pub system_program: Program<'info, System>,
}

pub fn process_init_spender(ctx: Context<InitializeSpender>) -> Result<()> {
    let spender = &mut ctx.accounts.spender;
    spender.multiplier = 1; 
    spender.points_earned = 0;
    Ok(())
}
