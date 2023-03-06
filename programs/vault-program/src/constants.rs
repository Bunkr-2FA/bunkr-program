use anchor_lang::prelude::*;

#[constant]
pub const USER_PROFILE_TAG: &[u8] = b"USER_PROFILE_STATE";
#[constant]
pub const MAX_TWITTER_HANDLE_LENGTH: usize = 15;
#[constant]
pub const POST_TAG: &[u8] = b"POST_STATE";
#[constant]
pub const MAX_TITLE_LENGTH: usize = 32;
#[constant]
pub const MAX_CONTENT_LENGTH: usize = 3000;