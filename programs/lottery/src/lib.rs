use anchor_lang::{
    prelude::*,
    solana_program::{
        account_info::AccountInfo, clock::Clock, hash::Hash, program::invoke,
        system_instruction::transfer,
    },
};
mod constants;
use constants::*;

declare_id!("HjXkH9s7uPhBWXGsJJ9LhwHQF326Jw8Jsz374yK1pcNp");

#[program]
pub mod lottery {
    use super::*;

    pub fn initialize(ctx: Context<InitMaster>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMaster<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR_SIZE + 4,
        seeds = [MASTER_SEED.as_bytes()],
        bump
    )]
    pub master: Account<'info, Master>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Master {
    pub last_id: u32,
}
