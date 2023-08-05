use {
    crate::{constants::*, errors::ErrorCode, states::validate_password, states::*},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct ResetRoot<'info> {
    #[account(mut, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    pub bunkr: Account<'info, Bunkr>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ResetRootObject {
    pub new_root: [u8; 32],
    pub new_init_time: u32,
    pub shadow_drive_space: String,
    pub reset_hash: [u8; 32],
}

pub fn handler(ctx: Context<ResetRoot>, reset_root_object: ResetRootObject) -> Result<()> {
    let bunkr = &mut ctx.accounts.bunkr;

    validate_password(bunkr.current_reset_hash, &reset_root_object.reset_hash)?;

    bunkr.root = reset_root_object.new_root;
    bunkr.init_time = reset_root_object.new_init_time;
    bunkr.last_accessed_interval = 0;
    bunkr.shadow_drive_space = reset_root_object.shadow_drive_space;
    bunkr.activated = false;
    msg!("Root reset initiated");
    Ok(())
}
