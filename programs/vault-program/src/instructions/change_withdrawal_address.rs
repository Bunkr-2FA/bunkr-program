use {
    crate::{constants::*, errors::ErrorCode, states::*},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct ChangeWithdrawalAddress<'info> {
    #[account(mut, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    pub bunkr: Account<'info, Bunkr>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account()]
    pub new_withdrawal_address: SystemAccount<'info>,
}

pub fn handler(ctx: Context<ChangeWithdrawalAddress>) -> Result<()> {
    let bunkr = &mut ctx.accounts.bunkr;
    let new_withdrawal_address = ctx.accounts.new_withdrawal_address.key();
    bunkr.withdraw_address = new_withdrawal_address;

    msg!("Withdrawal address changed");
    Ok(())
}
