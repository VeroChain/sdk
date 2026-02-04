use anchor_lang::prelude::*;
use crate::state::Petition;

#[derive(Accounts)]
pub struct TogglePrivacy<'info> {
    #[account(mut)]
    pub petition: Account<'info, Petition>,
}

pub fn toggle_privacy(ctx: Context<TogglePrivacy>) -> Result<()> {
    let petition = &mut ctx.accounts.petition;
    petition.private = !petition.private;
    Ok(())
}
