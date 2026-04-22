use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub bump: u8,
}

impl Vault {
    pub const LEN: usize = 8 + 1;
}