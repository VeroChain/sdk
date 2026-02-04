use anchor_lang::prelude::*;
use crate::state::Petition;

#[derive(Accounts)]
pub struct SignPetition<'info> {
    #[account(mut)]
    pub petition: Account<'info, Petition>,
}

pub fn sign_petition(ctx: Context<SignPetition>) -> Result<()> {
    ctx.accounts.petition.signatures += 1;
    Ok(())
}
