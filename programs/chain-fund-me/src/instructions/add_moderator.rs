use crate::{CrowdfundingError, Factory};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AddModerator<'info> {
    #[account(mut)]
    pub factory: Account<'info, Factory>,
    pub owner: Signer<'info>,
}

pub fn add_moderator(ctx: Context<AddModerator>, moderator: Pubkey) -> Result<()> {
    let factory = &mut ctx.accounts.factory;

    require!(
        factory.owner == ctx.accounts.owner.key(),
        CrowdfundingError::NotFactoryOwner
    );

    let exists = factory
        .moderators
        .iter()
        .any(|(key, _)| *key == moderator);

    if !exists {
        factory.moderators.push((moderator, true));
    }

    Ok(())
}
