use {
    crate::{states::*, constants::*, errors::ErrorCode},
    anchor_lang::{prelude::*}
};

#[derive(Accounts)]
pub struct TestWithdraw<'info> {
    #[account(mut, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    pub bunkr: Account<'info, Bunkr>,
    #[account(mut)]
    pub signer: Signer<'info>,
}


