use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The Twitter handle is too long. The maximum length is 15 characters.")]
    TwitterHandleTooLong,
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("The title is too long. The maximum length is 32 characters.")]
    TitleTooLong,
    #[msg("The content is too long. The maximum length is 3000 characters.")]
    ContentTooLong,
}