use anchor_lang::prelude::*;

#[error_code]
pub enum VeroError {
    #[msg("Petition is closed")]
    PetitionClosed,
}
