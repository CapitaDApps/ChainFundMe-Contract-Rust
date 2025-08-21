use crate::{events::Deposited, Campaign, MultiplierTier, Spender};
use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct InitializeSpender<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

   #[account(
        init_if_needed,
        payer = contributor,
        space = 8 + Spender::INIT_SPACE,
        seeds = [b"spender", contributor.key().as_ref()],
        bump
    )]
    pub spender: Account<'info, Spender>,
    pub system_program: Program<'info, System>
}

pub fn process_initialize_spender(ctx: Context<InitializeSpender>)->Result<()>{
   let spender = &mut ctx.accounts.spender;
   spender.multiplier= 0;
   spender.multiplier_tier = MultiplierTier::Base;
   spender.points_earned=0;

   Ok(())
}