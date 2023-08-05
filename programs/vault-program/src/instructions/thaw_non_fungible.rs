use anchor_spl::token::Transfer;

use {
    crate::{states::*, constants::*, errors::ErrorCode,},
    anchor_lang::{prelude::*,solana_program::program::invoke_signed},
    mpl_token_metadata::instruction::thaw_delegated_account,
    anchor_spl::{token::{Mint, Token, TokenAccount, Revoke, revoke,}, associated_token::AssociatedToken, metadata::Metadata}
};



#[derive(Accounts)]
pub struct ThawNonFungible<'info> {
    #[account(
        mut, 
        token::mint = token_mint, 
        token::authority = signer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    /// CHECK instruction will fail if wrong edition is supplied
    pub token_mint_edition: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token_mint, 
        associated_token::authority = withdrawal_address
    )]
    pub withdrawal_token_account: Account<'info, TokenAccount>,

    #[account(address = bunkr.withdraw_address)]
    pub withdrawal_address: SystemAccount<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    pub bunkr: Box<Account<'info, Bunkr>>,

    #[account(address = AUTHENTICATION_WALLET.parse::<Pubkey>().unwrap())]
    pub authentication_wallet: Signer<'info>,

    token_program: Program<'info, Token>,
    token_metadata_program: Program<'info, Metadata>,
    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
}


pub fn handler(ctx: Context<ThawNonFungible>) -> Result<()> {
    let signer = ctx.accounts.signer.key();
    let withdrawal_address = ctx.accounts.withdrawal_address.key();

    let seeds = &[
        b"bunkr",
        signer.as_ref(),
        &[*ctx.bumps.get("bunkr").unwrap()]
    ];

    let delegate_seeds = &[&seeds[..]];

    invoke_signed(
        &thaw_delegated_account(
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

        if withdrawal_address != signer {
            let cpi_accounts = Transfer {
                from: ctx.accounts.token_account.to_account_info(),
                to: ctx.accounts.withdrawal_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            };
        
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
            anchor_spl::token::transfer(cpi_context, 1)?;
            
        }

        let cpi_accounts = Revoke{
            source: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        revoke(cpi_context)?;


    Ok(())
}