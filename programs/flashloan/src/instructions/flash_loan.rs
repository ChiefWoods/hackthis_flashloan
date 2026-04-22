use anchor_lang::prelude::*;
use solana_instructions_sysvar::{
    load_current_index_checked, load_instruction_at_checked, ID as SYSVAR_INSTRUCTIONS_ID,
};
use crate::{constants::{FLASH_LOAN_DISCRIMINATOR, FLASH_REPAY_DISCRIMINATOR}, error::ErrorCode, Vault, VAULT_SEED};

#[derive(Accounts)]
pub struct FlashLoan<'info> {
    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,
    /// CHECK: This account receives the loan funds
    #[account(mut)]
    pub borrower: UncheckedAccount<'info>,
    /// CHECK: Instructions sysvar, verified by address constraint
    #[account(address = SYSVAR_INSTRUCTIONS_ID)]
    pub instructions: UncheckedAccount<'info>,
}

pub(crate) fn handler(ctx: Context<FlashLoan>, amount: u64) -> Result<()> {
    let ixs = ctx.accounts.instructions.to_account_info();
    let current_index = load_current_index_checked(&ixs)? as usize;

    // Scan all instructions: reject any other flash_loan, require a flash_repay after this one.
    // A second flash_loan in the same tx could drain the vault by sharing a single flash_repay
    // that only repays the smaller of the two loans.
    let mut found_repay = false;
    let mut idx = 0usize;
    loop {
        match load_instruction_at_checked(idx, &ixs) {
            Ok(ix) => {
                if ix.program_id == crate::id() && ix.data.len() >= 8 {
                    let disc = &ix.data[..8];
                    if idx != current_index && disc == FLASH_LOAN_DISCRIMINATOR {
                        return Err(ErrorCode::MultipleLoansNotAllowed.into());
                    }
                    if idx > current_index && disc == FLASH_REPAY_DISCRIMINATOR {
                        found_repay = true;
                    }
                }
                idx += 1;
            }
            Err(_) => break,
        }
    }
    require!(found_repay, ErrorCode::MissingRepayInstruction);

    // Transfer loan amount from vault to borrower via direct lamport manipulation.
    // Vault is program-owned, so we can mutate its lamports without a CPI.
    let vault_info = ctx.accounts.vault.to_account_info();
    let borrower_info = ctx.accounts.borrower.to_account_info();
    require!(
        vault_info.lamports() >= amount,
        ErrorCode::InsufficientVaultBalance
    );
    **vault_info.try_borrow_mut_lamports()? -= amount;
    **borrower_info.try_borrow_mut_lamports()? += amount;

    Ok(())
}