use crate::{CrowdfundingError, Factory, Moderators};
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
        .any(|m| m.moderator == moderator);

    if !exists {
        factory.moderators.push(Moderators {
            moderator,
            allowed: true,
        });
    }

    Ok(())
}
