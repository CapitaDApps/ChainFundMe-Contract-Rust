use anchor_lang::prelude::*;
use crate::{Factory, CrowdfundingError};

#[derive(Accounts)]
pub struct RemoveModerator<'info> {
    #[account(mut, has_one = owner)]
    pub factory: Account<'info, Factory>,
    pub owner: Signer<'info>,
}

pub fn remove_moderator(ctx: Context<RemoveModerator>, moderator: Pubkey) -> Result<()> {
    let factory = &mut ctx.accounts.factory;

    require!(
        factory.owner == ctx.accounts.owner.key(),
        CrowdfundingError::NotFactoryOwner
    );

    factory.moderators.retain(|key| key.moderator != moderator);

    Ok(())
}
