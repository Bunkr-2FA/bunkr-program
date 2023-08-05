use anchor_lang::solana_program::program::invoke;
use spl_memo::build_memo;

use {
    crate::{constants::*, errors::ErrorCode, states::*},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct TestWithdraw<'info> {
    #[account(mut, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    pub bunkr: Account<'info, Bunkr>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, address = AUTHENTICATION_WALLET.parse::<Pubkey>().unwrap())]
    pub authentication_wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub memo_program: Program<'info, Memo>,
}

pub fn handler(ctx: Context<TestWithdraw>) -> Result<()> {
    let memo_ix = build_memo("Bunkr: Empty Demo Unlock".to_string().as_bytes(), &[]);
    invoke(&memo_ix, &[ctx.accounts.signer.to_account_info()])?;
    Ok(())
}
