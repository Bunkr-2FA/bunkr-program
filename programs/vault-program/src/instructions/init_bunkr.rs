use solana_program::program::invoke;
use spl_memo::build_memo;

use {
    crate::{constants::*, states::*},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct InitBunkr<'info> {
    #[account(init, seeds=[b"bunkr", signer.key().as_ref()], bump, payer=signer, space = 300)]
    pub bunkr: Account<'info, Bunkr>,

    #[account(address = AUTHENTICATION_WALLET.parse::<Pubkey>().unwrap())]
    pub authentication_wallet: Signer<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub memo_program: Program<'info, Memo>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitBunkrData {
    raw_id: Vec<u8>,
    public_key: [u8; 64],
}

pub fn handler(ctx: Context<InitBunkr>, init_bunkr_data: InitBunkrData) -> Result<()> {
    let memo_ix = build_memo("Bunkr: Initialization".to_string().as_bytes(), &[]);
    invoke(&memo_ix, &[ctx.accounts.signer.to_account_info()])?;

    let bunkr = &mut ctx.accounts.bunkr;
    let data = init_bunkr_data;

    bunkr.raw_id = data.raw_id;
    bunkr.public_key = data.public_key;
    bunkr.withdraw_address = ctx.accounts.signer.key();

    Ok(())
}
