use {
    crate::{states::*, constants::*, errors::ErrorCode,},
    anchor_lang::{prelude::*,solana_program::program::invoke_signed},
    mpl_token_metadata::instruction::{freeze_delegated_account},
    anchor_spl::{token::{Mint, Token, TokenAccount, Approve, approve}}
};



#[derive(Accounts)]
pub struct FreezeNonFungible<'info> {
    #[account(
        mut, 
        token::mint = token_mint, 
        token::authority = signer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    /// CHECK instruction will fail if wrong edition is supplied
    pub token_mint_edition: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    pub bunkr: Account<'info, Bunkr>,

    token_program: Program<'info, Token>,
    /// CHECK intstruction will fail if wrong program is supplied
    token_metadata_program: AccountInfo<'info>
}







pub fn handler(ctx: Context<FreezeNonFungible>) -> Result<()> {

    let cpi_accounts = Approve{
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
        delegate: ctx.accounts.bunkr.to_account_info()
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    approve(cpi_context, 1)?;

    let signer = ctx.accounts.signer.key();
    
    let seeds = &[
        b"bunkr",
        signer.as_ref(),
        &[*ctx.bumps.get("bunkr").unwrap()]
    ];
    let delegate_seeds = &[&seeds[..]];

    invoke_signed(
    &freeze_delegated_account(
        mpl_token_metadata::id(),
        ctx.accounts.bunkr.key(),
        ctx.accounts.token_account.key(),
        ctx.accounts.token_mint_edition.key(),
        ctx.accounts.token_mint.key()
    ), 
    &[
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.bunkr.to_account_info(),
            ctx.accounts
                .token_account
                .to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_mint_edition.to_account_info(),
            ctx.accounts.token_mint.to_account_info(),
        ], 
    delegate_seeds)?;
Ok(())
}