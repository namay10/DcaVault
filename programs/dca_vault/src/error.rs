use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Amount must be greater than zero.")]
    InvalidAmount,
    #[msg("Periods must be greater than zero.")]
    InvalidPeriods,
    #[msg("Interval must be greater than zero.")]
    InvalidInterval,
    #[msg("Unauthorized access to vault.")]
    Unauthorized,
    #[msg("Swap is not due yet.")]
    SwapNotDue,
    #[msg("DCA plan is already complete.")]
    DcaPlanComplete,
    #[msg("Insufficient balance in vault.")]
    InsufficientBalance,
    #[msg("Invalid slice amount for swap.")]
    InvalidSliceAmount,
    #[msg("Arithmetic overflow occurred.")]
    ArithmeticOverflow,
    #[msg("Invalid Jupiter accounts provided.")]
    InvalidJupiterAccounts,
}
