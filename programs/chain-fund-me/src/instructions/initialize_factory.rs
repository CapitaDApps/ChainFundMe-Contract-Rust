use anchor_lang::prelude::*;

use crate::{AcceptedToken, Factory};

#[derive(Accounts)]
#[instruction(factory_id: u64)]
pub struct InitializeFactory<'info> {
    #[account(
        init_if_needed,
        payer = owner,
        space = Factory::space(),
        seeds = [b"factory", factory_id.to_le_bytes().as_ref()],
        bump
    )]
    pub factory: Account<'info, Factory>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn process_initialize_factory(
    ctx: Context<InitializeFactory>,
    factory_id: u64,
    platform_fee: u8,
    stablecoin_mint: Pubkey,
    fee_wallet: Pubkey,
    other_accepted_tokens: Vec<AcceptedToken>,
) -> Result<()> {
    let factory = &mut ctx.accounts.factory;
    factory.factory_id = factory_id;
    factory.owner = ctx.accounts.owner.key();
    factory.platform_fee = platform_fee;
    factory.stablecoin_mint = stablecoin_mint;
    factory.fee_wallet = fee_wallet;
    factory.deployed_campaigns_count = 0;
    factory.limits_enabled = false;
    factory.is_paused = false;

    factory.other_accepted_tokens = other_accepted_tokens
        .into_iter()
        .map(|t| AcceptedToken {
            mint: t.mint,
            allowed: t.allowed,
        }).collect();
    Ok(())
}
