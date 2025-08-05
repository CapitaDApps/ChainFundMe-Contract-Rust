use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct RemoveModerator<'info> {
    #[account(mut)]
    pub factory: Account<'info, FundingFactory>,
    pub owner: Signer<'info>,
}

pub fn remove_moderator(ctx: Context<RemoveModerator>, moderator: Pubkey) -> Result<()> {
        let factory = &mut ctx.accounts.factory;
        require!(factory.owner == ctx.accounts.owner.key(), CrowdfundingError::NotFactoryOwner);
        
        factory.moderators.retain(|&x| x != moderator);
        
        Ok(())
    }