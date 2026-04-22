use anchor_lang::prelude::*;
use crate::{Vault, VAULT_SEED};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = Vault::LEN,
        seeds = [VAULT_SEED],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.vault.bump = ctx.bumps.vault;
    msg!("Vault initialized: {}", ctx.accounts.vault.key());
    Ok(())
}