use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Transaction must include a flash_repay instruction")]
    MissingRepayInstruction,
    #[msg("Transaction must include a flash_loan instruction")]
    MissingLoanInstruction,
    #[msg("Insufficient funds in vault")]
    InsufficientVaultBalance,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Only one flash_loan is allowed per transaction")]
    MultipleLoansNotAllowed,
}