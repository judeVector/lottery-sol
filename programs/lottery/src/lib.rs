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
        msg!(
            "Greetings from: {:?}, account has been initiated",
            ctx.program_id
        );
        Ok(())
    }

    pub fn create_lotery(ctx: Context<CreateLottery>, ticket_price: u64) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        let master = &mut ctx.accounts.master;

        // Increment the last ticket id in master
        master.last_id += 1;

        // Set the lottery values
        lottery.id = master.last_id;
        lottery.authority = ctx.accounts.authority.key();
        lottery.ticket_price = ticket_price;

        msg!("Created lottery: {}", lottery.id);
        msg!("Authority: {}", lottery.authority);
        msg!("Ticket price: {}", lottery.ticket_price);

        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, lottery_id: u32) -> Result<()> {
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

#[derive(Accounts)]
pub struct CreateLottery<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR_SIZE + Lottery::INIT_SPACE,
        seeds = [LOTTERY_SEED.as_bytes(), &(master.last_id + 1).to_le_bytes()],
        bump
    )]
    pub lottery: Account<'info, Lottery>,
    #[account(
        mut,
        seeds = [MASTER_SEED.as_bytes()],
        bump
    )]
    pub master: Account<'info, Master>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(lottery_id: u32)]
pub struct BuyTicket<'info> {
    #[account(
        mut,
        seeds = [LOTTERY_SEED.as_bytes(), &lottery_id.to_le_bytes()],
        bump
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(
        init,
        payer = buyer,
        space = 8 + 4 + 4 + 32,
        seeds = [
            TICKET_SEED.as_bytes(),
            lottery.key().as_ref(),
            &(lottery.last_ticket_id + 1).to_le_bytes()
        ],
        bump

    )]
    pub ticket: Account<'info, Ticket>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Master {
    pub last_id: u32,
}

#[account]
#[derive(InitSpace)]
pub struct Lottery {
    pub id: u32,
    #[max_len(Pubkey)]
    pub authority: Pubkey,
    pub ticket_price: u64,
    pub last_ticket_id: u32,
    #[max_len(Option<u32>)]
    pub winner_id: Option<u32>,
    #[max_len(bool)]
    pub claimed: bool,
}

#[account]
pub struct Ticket {}
