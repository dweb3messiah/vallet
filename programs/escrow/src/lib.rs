/*use anchor_lang::prelude::*;

declare_id!("C5BAiC6khYmsshVtd5xQ1nU8xWtzAueMnSjeNWC5Fr3M");


#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}*/
use anchor_lang::prelude::*;
declare_id!("C5BAiC6khYmsshVtd5xQ1nU8xWtzAueMnSjeNWC5Fr3M");

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;

        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;

        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;

        Ok(())
    }
}

