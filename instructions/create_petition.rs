use anchor_lang::prelude::*;
use crate::state::Petition;

#[derive(Accounts)]
pub struct CreatePetition<'info> {
    #[account(init, payer = user, space = 8 + 128)]
    pub petition: Account<'info, Petition>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_petition(
    ctx: Context<CreatePetition>,
    title: String,
) -> Result<()> {
    let petition = &mut ctx.accounts.petition;
    petition.title = title;
    petition.signatures = 0;
    petition.private = false;
    Ok(())
}
