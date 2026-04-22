use anchor_lang::prelude::*;
use solana_instructions_sysvar::{
    load_current_index_checked, load_instruction_at_checked, ID as SYSVAR_INSTRUCTIONS_ID,
};
use crate::{
    constants::{FLASH_LOAN_DISCRIMINATOR, FLASH_LOAN_FEE},
    error::ErrorCode,
    Vault, VAULT_SEED,
};

#[derive(Accounts)]
pub struct FlashRepay<'info> {
    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub borrower: Signer<'info>,
    /// CHECK: Instructions sysvar, verified by address constraint
    #[account(address = SYSVAR_INSTRUCTIONS_ID)]
    pub instructions: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(ctx: Context<FlashRepay>) -> Result<()> {
    let ixs = ctx.accounts.instructions.to_account_info();
    let current_index = load_current_index_checked(&ixs)? as usize;

    // Find the flash_loan instruction that precedes this repay.
    // If multiple exist, the last one wins (each tx should have one loan/repay pair).
    let mut loan_amount: Option<u64> = None;
    for idx in 0..current_index {
        if let Ok(ix) = load_instruction_at_checked(idx, &ixs) {
            if ix.program_id == crate::id()
                && ix.data.len() >= 16
                && ix.data[..8] == FLASH_LOAN_DISCRIMINATOR
            {
                // Anchor encodes instruction arguments as little-endian borsh after the 8-byte discriminator
                loan_amount = Some(u64::from_le_bytes(ix.data[8..16].try_into().unwrap()));
            }
        }
    }

    let amount = loan_amount.unwrap_or(0);
    let repay_amount = amount.checked_add(FLASH_LOAN_FEE).ok_or(ErrorCode::Overflow)?;

    // Transfer principal + fee from borrower back to vault
    anchor_lang::system_program::transfer(
        CpiContext::new(
            anchor_lang::system_program::ID,
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.borrower.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        ),
        repay_amount,
    )?;

    Ok(())
}