pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("mqcAhBn6aLCnvGdQd12bG9pq6vL3Bj5A5Hmv3qYouGW");

#[program]
pub mod chain_fund_me {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
