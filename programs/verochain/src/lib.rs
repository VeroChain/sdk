use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;

declare_id!("Vero1111111111111111111111111111111111111");

#[program]
pub mod verochain {
    use super::*;

    pub fn create_petition(
        ctx: Context<CreatePetition>,
        title: String,
    ) -> Result<()> {
        instructions::create_petition(ctx, title)
    }

    pub fn sign_petition(ctx: Context<SignPetition>) -> Result<()> {
        instructions::sign_petition(ctx)
    }

    pub fn toggle_privacy(ctx: Context<TogglePrivacy>) -> Result<()> {
        instructions::toggle_privacy(ctx)
    }
}
