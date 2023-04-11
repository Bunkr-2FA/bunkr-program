use {
    crate::{states::*, constants::*, errors::ErrorCode},
    anchor_lang::{
        prelude::*, solana_program::program::invoke_signed,
        solana_program::program::invoke, 
        solana_program::instruction::Instruction
    },
    mpl_token_metadata::instruction::{MetadataInstruction, DelegateArgs, RevokeArgs, UnlockArgs, TransferArgs},
    anchor_spl::{token::{Mint, Token, TokenAccount}},
    anchor_spl::associated_token::{AssociatedToken}
};

#[derive(Accounts)]
pub struct UnlockPNFT<'info> {
    #[account(
        mut, 
        token::mint = token_mint, 
        token::authority = signer,
    )]
    pub token_account: Box<Account<'info, TokenAccount>>,
    pub token_mint: Box<Account<'info, Mint>>,
    /// CHECK instruction will fail if wrong edition is supplied
    pub token_mint_edition: AccountInfo<'info>,
    /// CHECK instruction will fail if wrong record is supplied
    pub token_mint_record: AccountInfo<'info>,
    /// CHECK instruction will fail if wrong metadata is supplied
    #[account(mut)]
    mint_metadata: UncheckedAccount<'info>,
    /// CHECK instruction will fail if wrong rules are supplied
    pub auth_rules: UncheckedAccount<'info>,
    /// CHECK instruction will fail if wrong sysvar ixns are supplied
    pub sysvar_instructions: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    pub bunkr: Box<Account<'info, Bunkr>>,

    #[account(
        init_if_needed,
        associated_token::mint = token_mint,
        associated_token::authority = withdrawal_address,
        payer = signer
    )]
    pub to_associated_token_account: Account<'info, TokenAccount>,
    
    /// CHECK instruction will fail if wrong mint record is supplied
    pub to_token_mint_record: AccountInfo<'info>,

    #[account(constraint = withdrawal_address.key() == bunkr.withdraw_address)]
    pub withdrawal_address: SystemAccount<'info>,
    

    pub token_program: Program<'info, Token>,
    /// CHECK intstruction will fail if wrong program is supplied
    pub token_metadata_program: AccountInfo<'info>,
    /// CHECK intstruction will fail if wrong program is supplied
    pub auth_rules_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>
}


pub fn handler(ctx: Context<UnlockPNFT>) -> Result<()> {
    let bunkr = &ctx.accounts.bunkr;
    let signer = ctx.accounts.signer.key();
    let withdrawal_address = ctx.accounts.withdrawal_address.key();

    if withdrawal_address != signer {assert!(withdrawal_address == ctx.accounts.bunkr.withdraw_address, "Mismatched Withdrawal Address")}

    let seeds = &[
        b"bunkr",
        signer.as_ref(),
        &[*ctx.bumps.get("bunkr").unwrap()]
    ];
    let delegate_seeds = &[&seeds[..]];
    
    invoke_signed(
        &Instruction {
            program_id: mpl_token_metadata::id(),
            accounts: vec![
                // 0. `[signer]` Delegate
                AccountMeta::new_readonly(bunkr.key(), true),
                // 1. `[optional]` Token owner
                AccountMeta::new_readonly(ctx.accounts.signer.key(), false),
                // 2. `[mut]` Token account
                AccountMeta::new(ctx.accounts.token_account.key(), false),
                // 3. `[]` Mint account
                AccountMeta::new_readonly(ctx.accounts.token_mint.key(), false),
                // 4. `[mut]` Metadata account
                AccountMeta::new(ctx.accounts.mint_metadata.key(), false),
                // 5. `[optional]` Edition account
                AccountMeta::new_readonly(ctx.accounts.token_mint_edition.key(), false),
                // 6. `[optional, mut]` Token record account
                AccountMeta::new(ctx.accounts.token_mint_record.key(), false),
                // 7. `[signer, mut]` Payer
                AccountMeta::new(ctx.accounts.signer.key(), true),
                // 8. `[]` System Program
                AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
                // 9. `[]` Instructions sysvar account
                AccountMeta::new_readonly(ctx.accounts.sysvar_instructions.key(), false),
                // 10. `[optional]` SPL Token Program
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                // 11. `[optional]` Token Authorization Rules program
                AccountMeta::new_readonly(ctx.accounts.auth_rules_program.key(), false),
                // 12. `[optional]` Token Authorization Rules account
                AccountMeta::new_readonly(ctx.accounts.auth_rules.key(), false),
            ],
            data: MetadataInstruction::Unlock(UnlockArgs::V1 { authorization_data: None }).try_to_vec().unwrap(),
        },
        &[
            bunkr.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.token_mint.to_account_info(),
            ctx.accounts.mint_metadata.to_account_info(),
            ctx.accounts.token_mint_edition.to_account_info(),
            ctx.accounts.token_mint_record.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.sysvar_instructions.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.auth_rules_program.to_account_info(),
            ctx.accounts.auth_rules.to_account_info(),
        ],
        delegate_seeds,
    )?;

    invoke(
        &Instruction {
            program_id: mpl_token_metadata::id(),
            accounts: vec![
                // #[account(0, optional, writable, name="delegate_record", desc="Delegate record account")]
                AccountMeta::new_readonly(mpl_token_metadata::id(), false),
                // #[account(1, name="delegate", desc="Owner of the delegated account")]
                AccountMeta::new_readonly(bunkr.key(), false),
                // #[account(2, writable, name = "metadata", desc = "Metadata account")]
                AccountMeta::new(ctx.accounts.mint_metadata.key(), false),
                // #[account(3, optional, name = "master_edition", desc = "Master Edition account")]
                AccountMeta::new_readonly(ctx.accounts.token_mint_edition.key(), false),
                // #[account(4, optional, writable, name = "token_record", desc = "Token record account")]
                AccountMeta::new(ctx.accounts.token_mint_record.key(), false),
                // #[account(5, name = "mint", desc = "Mint of metadata")]
                AccountMeta::new_readonly(ctx.accounts.token_mint.key(), false),
                // #[account(6, optional, writable, name = "token", desc = "Token account of mint")]
                AccountMeta::new(ctx.accounts.token_account.key(), false),
                // #[account(7, signer, name = "authority", desc = "Update authority or token owner")]
                AccountMeta::new_readonly(ctx.accounts.signer.key(), true),
                // #[account(8, signer, writable, name = "payer", desc = "Payer")]
                AccountMeta::new(ctx.accounts.signer.key(), true),
                // #[account(9, name = "system_program", desc = "System Program")]
                AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
                // #[account(10, name = "sysvar_instructions", desc = "Instructions sysvar account")]
                AccountMeta::new_readonly(ctx.accounts.sysvar_instructions.key(), false),
                // #[account(11, optional, name = "spl_token_program", desc = "SPL Token Program")]
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                // #[account(12, optional, name = "authorization_rules_program", desc = "Token Authorization Rules Program")]
                AccountMeta::new_readonly(ctx.accounts.auth_rules_program.key(), false),
                // #[account(13, optional, name = "authorization_rules", desc = "Token Authorization Rules account")]
                AccountMeta::new_readonly(ctx.accounts.auth_rules.key(), false),
            ],
            data: MetadataInstruction::Revoke(RevokeArgs::StakingV1).try_to_vec().unwrap(),
        },
        &[
            bunkr.to_account_info(),
            ctx.accounts.mint_metadata.to_account_info(),
            ctx.accounts.token_mint_edition.to_account_info(),
            ctx.accounts.token_mint_record.to_account_info(),
            ctx.accounts.token_mint.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.sysvar_instructions.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.auth_rules_program.to_account_info(),
            ctx.accounts.auth_rules.to_account_info(),
        ],
    )?;
        if withdrawal_address != signer {
            assert!(withdrawal_address == ctx.accounts.bunkr.withdraw_address);

            invoke(
                &Instruction {
                    program_id: mpl_token_metadata::id(),
                    accounts: vec![
                        // 0. `[writable]` Delegate record account
                        AccountMeta::new_readonly(mpl_token_metadata::id(), false),
                        // 1. `[]` Delegated owner
                        AccountMeta::new_readonly(bunkr.key(), false),
                        // 2. `[writable]` Metadata account
                        AccountMeta::new(ctx.accounts.mint_metadata.key(), false),
                        // 3. `[optional]` Master Edition account
                        AccountMeta::new_readonly(ctx.accounts.token_mint_edition.key(), false),
                        // 4. `[]` Token record
                        AccountMeta::new(ctx.accounts.token_mint_record.key(), false),
                        // 5. `[]` Mint account
                        AccountMeta::new_readonly(ctx.accounts.token_mint.key(), false),
                        // 6. `[optional, writable]` Token account
                        AccountMeta::new(ctx.accounts.token_account.key(), false),
                        // 7. `[signer]` Approver (update authority or token owner) to approve the delegation
                        AccountMeta::new_readonly(ctx.accounts.signer.key(), true),
                        // 8. `[signer, writable]` Payer
                        AccountMeta::new(ctx.accounts.signer.key(), true),
                        // 9. `[]` System Program
                        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
                        // 10. `[]` Instructions sysvar account
                        AccountMeta::new_readonly(ctx.accounts.sysvar_instructions.key(), false),
                        // 11. `[optional]` SPL Token Program
                        AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                        // 12. `[optional]` Token Authorization Rules program
                        AccountMeta::new_readonly(ctx.accounts.auth_rules_program.key(), false),
                        // 13. `[optional]` Token Authorization Rules account
                        AccountMeta::new_readonly(ctx.accounts.auth_rules.key(), false),
                    ],
                    data: MetadataInstruction::Delegate(DelegateArgs::TransferV1{
                        amount: 1,
                        authorization_data: None,
                    })
                    .try_to_vec()
                    .unwrap(),
                },
                &[
                    bunkr.to_account_info(),
                    ctx.accounts.mint_metadata.to_account_info(),
                    ctx.accounts.token_mint_edition.to_account_info(),
                    ctx.accounts.token_mint_record.to_account_info(),
                    ctx.accounts.token_mint.to_account_info(),
                    ctx.accounts.token_account.to_account_info(),
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.sysvar_instructions.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.auth_rules_program.to_account_info(),
                    ctx.accounts.auth_rules.to_account_info(),
                ],
            )?;

            invoke_signed(
                &Instruction {
                    program_id: mpl_token_metadata::id(),
                    accounts: vec![
                        // #[account(0, writable, name="token", desc="Token account")]
                        AccountMeta::new(ctx.accounts.token_account.key(), false),
                        // #[account(1, name="token_owner", desc="Token account owner")]
                        AccountMeta::new_readonly(signer.key(), false),
                        // #[account(2, writable, name="destination", desc="Destination token account")]
                        AccountMeta::new(ctx.accounts.to_associated_token_account.key(), false),
                        // #[account(3, name="destination_owner", desc="Destination token account owner")]
                        AccountMeta::new_readonly(ctx.accounts.withdrawal_address.key(), false),
                        // #[account(4, name="mint", desc="Mint of token asset")]
                        AccountMeta::new_readonly(ctx.accounts.token_mint.key(), false),
                        // #[account(5, writable, name="metadata", desc="Metadata (pda of ['metadata', program id, mint id])")]
                        AccountMeta::new(ctx.accounts.mint_metadata.key(), false),
                        // #[account(6, optional, name="edition", desc="Edition of token asset")]
                        AccountMeta::new_readonly(ctx.accounts.token_mint_edition.key(), false),
                        // #[account(7, optional, writable, name="recipient_token_record", desc="Owner token record account")]
                        AccountMeta::new(ctx.accounts.token_mint_record.key(), false),
                        // #[account(8, optional, writable, name="destination_token_record", desc="Destination token record account")]
                        AccountMeta::new(ctx.accounts.to_token_mint_record.key(), false),
                        // #[account(9, signer, name="authority", desc="Transfer authority (token owner or delegate)")]
                        AccountMeta::new_readonly(bunkr.key(), true),
                        // #[account(10, signer, writable, name="payer", desc="Payer")]
                        AccountMeta::new(ctx.accounts.signer.key(), true),
                        // #[account(11, name="system_program", desc="System Program")]
                        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
                        // #[account(12, name="sysvar_instructions", desc="Instructions sysvar account")]
                        AccountMeta::new_readonly(ctx.accounts.sysvar_instructions.key(), false),
                        // #[account(13, name="spl_token_program", desc="SPL Token Program")]
                        AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                        // #[account(14, name="spl_ata_program", desc="SPL Associated Token Account program")]
                        AccountMeta::new_readonly(ctx.accounts.associated_token_program.key(), false),
                        // #[account(15, optional, name="authorization_rules_program", desc="Token Authorization Rules Program")]
                        AccountMeta::new_readonly(ctx.accounts.auth_rules_program.key(), false),
                        // #[account(16, optional, name="authorization_rules", desc="Token Authorization Rules account")]
                        AccountMeta::new_readonly(ctx.accounts.auth_rules.key(), false),
                    ],
                    data: MetadataInstruction::Transfer(TransferArgs::V1 {
                        amount: 1,
                        authorization_data: None,
                    })
                    .try_to_vec()
                    .unwrap(),
                },
                &[
                    ctx.accounts.token_account.to_account_info(),
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.to_associated_token_account.to_account_info(),
                    ctx.accounts.withdrawal_address.to_account_info(),
                    ctx.accounts.token_mint.to_account_info(),
                    ctx.accounts.mint_metadata.to_account_info(),
                    ctx.accounts.token_mint_edition.to_account_info(),
                    ctx.accounts.token_mint_record.to_account_info(),
                    ctx.accounts.to_token_mint_record.to_account_info(),
                    ctx.accounts.bunkr.to_account_info(),
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.sysvar_instructions.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.associated_token_program.to_account_info(),
                    ctx.accounts.auth_rules_program.to_account_info(),
                    ctx.accounts.auth_rules.to_account_info(),
                ],
                delegate_seeds,
            )?;

            invoke(
                &Instruction {
                    program_id: mpl_token_metadata::id(),
                    accounts: vec![
                        // #[account(0, optional, writable, name="delegate_record", desc="Delegate record account")]
                        AccountMeta::new_readonly(mpl_token_metadata::id(), false),
                        // #[account(1, name="delegate", desc="Owner of the delegated account")]
                        AccountMeta::new_readonly(bunkr.key(), false),
                        // #[account(2, writable, name = "metadata", desc = "Metadata account")]
                        AccountMeta::new(ctx.accounts.mint_metadata.key(), false),
                        // #[account(3, optional, name = "master_edition", desc = "Master Edition account")]
                        AccountMeta::new_readonly(ctx.accounts.token_mint_edition.key(), false),
                        // #[account(4, optional, writable, name = "token_record", desc = "Token record account")]
                        AccountMeta::new(ctx.accounts.token_mint_record.key(), false),
                        // #[account(5, name = "mint", desc = "Mint of metadata")]
                        AccountMeta::new_readonly(ctx.accounts.token_mint.key(), false),
                        // #[account(6, optional, writable, name = "token", desc = "Token account of mint")]
                        AccountMeta::new(ctx.accounts.token_account.key(), false),
                        // #[account(7, signer, name = "authority", desc = "Update authority or token owner")]
                        AccountMeta::new_readonly(ctx.accounts.signer.key(), true),
                        // #[account(8, signer, writable, name = "payer", desc = "Payer")]
                        AccountMeta::new(ctx.accounts.signer.key(), true),
                        // #[account(9, name = "system_program", desc = "System Program")]
                        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
                        // #[account(10, name = "sysvar_instructions", desc = "Instructions sysvar account")]
                        AccountMeta::new_readonly(ctx.accounts.sysvar_instructions.key(), false),
                        // #[account(11, optional, name = "spl_token_program", desc = "SPL Token Program")]
                        AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                        // #[account(12, optional, name = "authorization_rules_program", desc = "Token Authorization Rules Program")]
                        AccountMeta::new_readonly(ctx.accounts.auth_rules_program.key(), false),
                        // #[account(13, optional, name = "authorization_rules", desc = "Token Authorization Rules account")]
                        AccountMeta::new_readonly(ctx.accounts.auth_rules.key(), false),
                    ],
                    data: MetadataInstruction::Revoke(RevokeArgs::TransferV1).try_to_vec().unwrap(),
                },
                &[
                    bunkr.to_account_info(),
                    ctx.accounts.mint_metadata.to_account_info(),
                    ctx.accounts.token_mint_edition.to_account_info(),
                    ctx.accounts.token_mint_record.to_account_info(),
                    ctx.accounts.token_mint.to_account_info(),
                    ctx.accounts.token_account.to_account_info(),
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.sysvar_instructions.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.auth_rules_program.to_account_info(),
                    ctx.accounts.auth_rules.to_account_info(),
                ],
            )?;
            
            
        }


    Ok(())
}