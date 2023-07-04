

use {
    anchor_lang::{prelude::*},
    crate::{constants::*, errors::ErrorCode, states::*},
    crate::states::{validate_password, validate_root, AuthenticationObject}
};

#[derive(Accounts)]
pub struct ConfirmRoot<'info> {
    #[account(mut, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    pub bunkr: Account<'info, Bunkr>,
    #[account(mut)]
    pub signer: Signer<'info>,
}


pub fn handler(ctx: Context<ConfirmRoot>, confirm_root_object: AuthenticationObject ) -> Result<()> {
    let bunkr = &mut ctx.accounts.bunkr;
    let current_time_interval = Clock::get()?.unix_timestamp / 30;
    
    validate_password(bunkr.current_hash, &confirm_root_object.password_hash)?;
    validate_root(bunkr.init_time, &current_time_interval, bunkr.root.to_vec(), confirm_root_object.proof_path, confirm_root_object.otp_hash)?;

    bunkr.activated = true;
    
    msg!("Root confirmed; Bunkr Activated");
    Ok(())
}