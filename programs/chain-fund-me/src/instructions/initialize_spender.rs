// use anchor_lang::prelude::*;

// use crate::{Factory, Spender};

// #[derive(Accounts)]
// pub struct InitializePoints<'info> {
//     #[account(
//         init,
//         payer = owner,
//         space = 8 + 32 + 32 + 8 + 1,
//         seeds = [b"spender"],
//         bump
//     )]
//     pub points_config: Account<'info, Spender>,
//     pub factory: Account<'info, Factory>,
//     #[account(mut)]
//     pub owner: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

