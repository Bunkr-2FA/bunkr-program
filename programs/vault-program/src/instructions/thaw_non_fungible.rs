use {
    crate::{states::*, constants::*, errors::ErrorCode,},
    anchor_lang::{prelude::*,solana_program::program::invoke_signed},
    mpl_token_metadata::instruction::{thaw_delegated_account},
    anchor_spl::{token::{Mint, Token, TokenAccount, Revoke, revoke}}
};



#[derive(Accounts)]
pub struct ThawNonFungible<'info> {
    #[account(
        mut, 
        token::mint = token_mint, 
        token::authority = signer
    )]
    token_account: Account<'info, TokenAccount>,
    token_mint: Account<'info, Mint>,
    /// CHECK instruction will fail if wrong edition is supplied
    token_mint_edition: AccountInfo<'info>,
    #[account(mut)]
    signer: Signer<'info>,

    #[account(mut, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    delegate: Account<'info, Bunkr>,

    token_program: Program<'info, Token>,
    /// CHECK intstruction will fail if wrong program is supplied
    token_metadata_program: AccountInfo<'info>

}





pub fn handler(ctx: Context<ThawNonFungible>) -> Result<()> {
    let signer = ctx.accounts.signer.key();

    let seeds = &[
        b"bunkr",
        signer.as_ref(),
        &[*ctx.bumps.get("delegate").unwrap()]
    ];

    let delegate_seeds = &[&seeds[..]];

    invoke_signed(
        &thaw_delegated_account(
            mpl_token_metadata::id(),
            ctx.accounts.delegate.key(),
            ctx.accounts.token_account.key(),
            ctx.accounts.token_mint_edition.key(),
            ctx.accounts.token_mint.key()
        ), 
        &[
                ctx.accounts.signer.to_account_info(),
                ctx.accounts.delegate.to_account_info(),
                ctx.accounts
                    .token_account
                    .to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.token_mint_edition.to_account_info(),
                ctx.accounts.token_mint.to_account_info(),
            ], 
        delegate_seeds)?;

        let cpi_accounts = Revoke{
            source: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        revoke(cpi_context)?;


    Ok(())
}