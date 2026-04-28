use anchor_lang::prelude::*;
use solana_instructions_sysvar::ID as SYSVAR_INSTRUCTIONS_ID;

#[derive(Accounts)]
pub struct Drain<'info> {
    /// CHECK: flashloan vault PDA, validated by the flashloan program
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,
    /// Receives the loan — must sign the top-level tx so flash_repay can debit them
    #[account(mut)]
    pub borrower: Signer<'info>,
    /// CHECK: instructions sysvar — flash_loan reads this to find the flash_repay
    #[account(address = SYSVAR_INSTRUCTIONS_ID)]
    pub instructions: UncheckedAccount<'info>,
    /// CHECK: the flashloan program
    #[account(address = flashloan::ID)]
    pub flashloan_program: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<Drain>, amount: u64) -> Result<()> {
    flashloan::cpi::flash_loan(
        CpiContext::new(
            ctx.accounts.flashloan_program.key(),
            flashloan::cpi::accounts::FlashLoan {
                vault: ctx.accounts.vault.to_account_info(),
                borrower: ctx.accounts.borrower.to_account_info(),
                instructions: ctx.accounts.instructions.to_account_info(),
            },
        ),
        amount,
    )
}
