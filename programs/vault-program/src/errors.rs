use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Name too long")]
    NameTooLong
}