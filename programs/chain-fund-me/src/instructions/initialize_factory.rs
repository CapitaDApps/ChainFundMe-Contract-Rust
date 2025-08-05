use anchor_lang::prelude::*;
use crate::state::factory::Factory;

#[derive(Accounts)]
pub struct InitializeFactory<'info> {
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + 32 + 1 + 32 + 32 + 8 + 1 + 1 + 1024,
        seeds = [b"factory"],
        bump
    )]
    pub factory: Account<'info, Factory>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

   pub fn process_initialize_factory(
        ctx: Context<InitializeFactory>,
        platform_fee: u8,
        stablecoin_mint: Pubkey,
        fee_wallet: Pubkey,
    ) -> Result<()> {
        let factory = &mut ctx.accounts.factory;
        factory.owner = ctx.accounts.owner.key();
        factory.platform_fee = platform_fee;
        factory.stablecoin_mint = stablecoin_mint;
        factory.fee_wallet = fee_wallet;
        factory.deployed_campaigns_count = 0;
        factory.limits_enabled = false;
        factory.is_paused = false;
        Ok(())
    }

