use anchor_lang::solana_program::program::invoke;
use spl_memo::build_memo;

use {
    crate::{states::*,constants::*, errors::ErrorCode},
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount, Transfer},
    anchor_spl::associated_token::AssociatedToken
};

#[derive(Accounts)]
pub struct UnlockFungible<'info> {
    #[account(
        mut, 
        associated_token::mint = token_mint, 
        associated_token::authority = bunkr
    )]
    pub from_associated_token_account: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        associated_token::mint = token_mint,
        associated_token::authority = withdrawal_address,
        payer = signer
    )]
    pub to_associated_token_account: Account<'info, TokenAccount>,
    
    #[account(address = bunkr.withdraw_address)]
    pub withdrawal_address: SystemAccount<'info>,
    #[account(mut)]
    signer: Signer<'info>,

    #[account(mut, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    pub bunkr: Account<'info, Bunkr>,

    #[account(address = AUTHENTICATION_WALLET.parse::<Pubkey>().unwrap())]
    pub authentication_wallet: Signer<'info>,

    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
    memo_program: Program<'info, Memo>,

}


pub fn handler(ctx: Context<UnlockFungible>, amount: u64) -> Result<()> {
    let memo_ix = build_memo("Bunkr: Unlock Token".to_string().as_bytes(), &[]);
    invoke(&memo_ix, &[ctx.accounts.signer.to_account_info()])?;


    let cpi_accounts = Transfer {
        from: ctx.accounts.from_associated_token_account.to_account_info(),
        to: ctx.accounts.to_associated_token_account.to_account_info(),
        authority: ctx.accounts.bunkr.to_account_info(),
    };
    
    let signer = ctx.accounts.signer.key();
    let seeds = &[
        b"bunkr",
        signer.as_ref(),
        &[*ctx.bumps.get("bunkr").unwrap()]
    ];

    let vault_seeds = &[&seeds[..]];

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(vault_seeds);

    anchor_spl::token::transfer(cpi_context, amount)?;
    

    Ok(())
}