use anchor_lang::prelude::*;

#[account]
pub struct Petition {
    pub title: String,
    pub signatures: u64,
    pub private: bool,
}
