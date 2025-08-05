use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct AddModerator<'info> {
    #[account(mut)]
    pub factory: Account<'info, FundingFactory>,
    pub owner: Signer<'info>,
}



pub fn add_moderator(ctx: Context<AddModerator>, moderator: Pubkey) -> Result<()> {
        let factory = &mut ctx.accounts.factory;
        require!(factory.owner == ctx.accounts.owner.key(), CrowdfundingError::NotFactoryOwner);
        
        if !factory.moderators.contains(&moderator) {
            factory.moderators.push(moderator);
        }
        
        Ok(())
    }
