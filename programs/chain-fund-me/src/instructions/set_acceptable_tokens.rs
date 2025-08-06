use anchor_lang::prelude::*;
use crate::{CrowdfundingError};
use crate::state::Factory;

#[derive(Accounts)]
pub struct SetAcceptableToken<'info> {
    #[account(mut, has_one = owner)]
    pub factory: Account<'info, Factory>,
    pub owner: Signer<'info>,
}

pub fn set_acceptable_token(
    ctx: Context<SetAcceptableToken>,
    token_mint: Pubkey,
    accepted: bool,
) -> Result<()> {
    let factory = &mut ctx.accounts.factory;

    require!(
        factory.owner == ctx.accounts.owner.key(),
        CrowdfundingError::NotFactoryOwner
    );

    if accepted {
        if !factory.other_accepted_tokens.iter().any(|(mint, _)| *mint == token_mint) {
            factory.other_accepted_tokens.push((token_mint, true));
        }
    } else {
        factory.other_accepted_tokens.retain(|(mint, _)| *mint != token_mint);
    }

    Ok(())
}

