use anchor_lang::prelude::*;

use crate::{Factory, PointsConfig};

#[derive(Accounts)]
pub struct InitializePoints<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 8 + 1,
        seeds = [b"points_config"],
        bump
    )]
    pub points_config: Account<'info, PointsConfig>,
    pub factory: Account<'info, Factory>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}



    pub fn initialize_points(ctx: Context<InitializePoints>, base_points: u64) -> Result<()> {
        let points_config = &mut ctx.accounts.points_config;
        points_config.owner = ctx.accounts.owner.key();
        points_config.factory = ctx.accounts.factory.key();
        points_config.base_points = base_points;
        points_config.paused = false;
        
        Ok(())
    }