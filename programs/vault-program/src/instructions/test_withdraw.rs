use anchor_lang::system_program::{Transfer, transfer};

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
    #[account(mut, constraint = authentication_wallet.key() == AUTHENTICATION_WALLET.parse::<Pubkey>().unwrap())]
    pub authentication_wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<TestWithdraw>) -> Result<()> {

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.bunkr.to_account_info(),
            to: ctx.accounts.signer.to_account_info(),
        },
    );

    transfer(cpi_context, 10000000)?;

Ok(())
}