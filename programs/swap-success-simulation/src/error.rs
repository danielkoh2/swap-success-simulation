use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Slippage Error")]
    SlippageError,
    #[msg("Min profit Error")]
    MinProfitError,
}