use anchor_lang::system_program::{Transfer, transfer};

use {
    crate::{constants::*, errors::ErrorCode, states::*},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct InitBunkr<'info> {
    #[account(init, seeds=[b"bunkr", signer.key().as_ref()], bump , payer=signer, space = 300)]
    pub bunkr: Account<'info,Bunkr>,

    #[account(mut, constraint = authentication_wallet.key() == AUTHENTICATION_WALLET.parse::<Pubkey>().unwrap())]
    pub authentication_wallet: Signer<'info>,
       
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitBunkrData {
    raw_id: Vec<u8>,
    public_key: [u8; 32],
    withdraw_address: Pubkey,
    current_reset_hash: [u8; 32],
    final_reset_hash: [u8; 32],

}

pub fn handler(ctx: Context<InitBunkr>, init_bunkr_data: InitBunkrData) -> Result<()> {
    let bunkr = &mut ctx.accounts.bunkr;
    let data = init_bunkr_data;

    bunkr.raw_id = data.raw_id;
    bunkr.public_key = data.public_key;
    bunkr.withdraw_address = ctx.accounts.signer.key();
    bunkr.current_reset_hash = data.current_reset_hash;
    bunkr.final_reset_hash = data.final_reset_hash;



    Ok(())
}