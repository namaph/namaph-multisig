use anchor_lang::prelude::*;

#[error_code]
pub enum NamaphError {
    #[msg("string is too long")]
    StringTooLong,
    #[msg("bump do not match")]
    BumpMismatch,
    #[msg("not enough balance")]
    NotEnoughBalanceInTreasury
}
