use anchor_lang::prelude::*;
use anchor_lang::Accounts;

pub fn process(_ctx: Context<Initialize>) -> Result<()> {
    msg!("Hello, world!");
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
}
