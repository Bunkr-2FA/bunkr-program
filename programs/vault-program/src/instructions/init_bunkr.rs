use {
    crate::{constants::*, errors::ErrorCode, states::*},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct InitBunkr<'info> {
    #[account(init, seeds=[b"bunkr", signer.key().as_ref()], bump , payer=signer, space = 300)]
    pub bunkr: Account<'info,Bunkr>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitBunkrData {
    pub name: String,
    pub init_time: u32,
    pub root: [u8; 32],
    pub initial_hash: [u8; 32],
    pub final_hash: [u8; 32],
    pub initial_reset_hash: [u8; 32],
    pub final_reset_hash: [u8; 32],
    pub shadow_drive_space: String
}

pub fn handler(ctx: Context<InitBunkr>, init_bunkr_data: InitBunkrData) -> Result<()> {
    let bunkr = &mut ctx.accounts.bunkr;
    let data = init_bunkr_data;
    bunkr.name = data.name;
    bunkr.withdraw_address = ctx.accounts.signer.key();
    bunkr.init_time = data.init_time ;
    bunkr.last_accessed = data.init_time;
    bunkr.root = data.root;
    bunkr.current_hash = data.initial_hash;
    bunkr.final_hash = data.final_hash;
    bunkr.current_reset_hash = data.initial_reset_hash;
    bunkr.final_reset_hash = data.final_reset_hash;
    bunkr.shadow_drive_space = data.shadow_drive_space;

    Ok(())
}