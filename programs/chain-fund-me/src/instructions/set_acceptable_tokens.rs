use anchor_lang::prelude::*;

use crate::Factory;

#[derive(Accounts)]
pub struct SetAcceptableToken<'info> {
    #[account(mut)]
    pub factory: Account<'info, Factory>,
    pub owner: Signer<'info>,
}

   
   pub fn set_acceptable_token(ctx: Context<SetAcceptableToken>, token_mint: Pubkey, accepted: bool) -> Result<()> {
        let factory = &mut ctx.accounts.factory;
        require!(factory.owner == ctx.accounts.owner.key(), CrowdfundingError::NotFactoryOwner);
        
        if accepted {
            if !factory.accepted_tokens.contains(&token_mint) {
                factory.accepted_tokens.push(token_mint);
            }
        } else {
            factory.accepted_tokens.retain(|&x| x != token_mint);
        }
        
        Ok(())
    }