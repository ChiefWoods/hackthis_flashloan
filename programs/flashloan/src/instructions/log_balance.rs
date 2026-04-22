use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct LogBalance<'info> {
    /// CHECK: read-only account whose lamport balance we log
    pub account: UncheckedAccount<'info>,
}

pub(crate) fn handler(ctx: Context<LogBalance>) -> Result<()> {
    let lamports = ctx.accounts.account.lamports();
    msg!(
        "Account {} balance: {} lamports ({} SOL)",
        ctx.accounts.account.key(),
        lamports,
        lamports as f64 / 1_000_000_000.0,
    );
    Ok(())
}
