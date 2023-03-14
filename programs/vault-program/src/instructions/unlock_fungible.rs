use {
    crate::{states::*,constants::*, errors::ErrorCode},
    anchor_lang::{prelude::*},
    anchor_spl::{token::{Mint, Token, TokenAccount, Transfer}},
    anchor_spl::associated_token::{AssociatedToken}
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
    
    #[account(constraint = withdrawal_address.key() == bunkr.withdraw_address)]
    pub withdrawal_address: SystemAccount<'info>,
    #[account(mut)]
    signer: Signer<'info>,

    #[account(mut, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    pub bunkr: Account<'info, Bunkr>,

    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>


}


pub fn handler(ctx: Context<UnlockFungible>, amount: u64) -> Result<()> {

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

    let raw_amount = amount * (10_usize.pow(ctx.accounts.token_mint.decimals as u32)) as u64;
    anchor_spl::token::transfer(cpi_context, raw_amount)?;

    Ok(())
}