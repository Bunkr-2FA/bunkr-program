use crate::{constants::*, errors::ErrorCode, states::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseBunkr<'info> {
    #[account(mut, close = signer, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    pub bunkr: Account<'info, Bunkr>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CloseBunkr>) -> Result<()> {
    Ok(())
}
