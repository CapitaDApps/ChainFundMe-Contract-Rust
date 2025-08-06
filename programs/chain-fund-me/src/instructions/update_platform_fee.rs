use anchor_lang::prelude::*;
use crate::CrowdfundingError;
use crate::Factory;

#[derive(Accounts)]
pub struct UpdatePlatformFee<'info> {
    #[account(mut)]
    pub factory: Account<'info, Factory>,
    pub owner: Signer<'info>,
}


pub fn update_platform_fee(ctx: Context<UpdatePlatformFee>, new_fee: u8) -> Result<()> {
        require!(new_fee >= 1 && new_fee <= 20, CrowdfundingError::InvalidPlatformFee);
        
        let factory = &mut ctx.accounts.factory;
        require!(factory.owner == ctx.accounts.owner.key(), CrowdfundingError::NotFactoryOwner);
        
        factory.platform_fee = new_fee;
        
        Ok(())
    }


