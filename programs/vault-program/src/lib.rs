
use anchor_spl::{token::{Mint, Token, TokenAccount, Approve, Revoke, Transfer, approve, revoke}};
use mpl_token_metadata::instruction::{freeze_delegated_account, thaw_delegated_account};
use anchor_lang::{prelude::*, solana_program::hash::Hash, solana_program::program::invoke_signed};
use anchor_spl::associated_token::{AssociatedToken};
//use bs58::{decode, encode};

declare_id!("3PHnbmYZU1zVmD3rrLbkFTQonrRFj2GwRZwnLHJunAfL");

#[program]
pub mod vault_program {


    use super::*;

    pub fn init_bunkr(ctx: Context<InitBunkr> , init_bunkr_data: InitBunkrData ) -> Result<()> {
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

    pub fn close_bunkr(ctx: Context<CloseBunkr>) -> Result<()> {
        Ok(())
    }

    pub fn test_withdraw(ctx: Context<TestWithdraw>,password_hash: Vec<u8>, otp_hash: Vec<u8>, proof_path: Vec<HashTuple> ) -> Result<()> {
        let bunkr = &mut ctx.accounts.bunkr;
        let root = bunkr.root;
        let current_time = Clock::get()?.unix_timestamp;
        validate_time(bunkr.last_accessed, &current_time)?;
        validate_root(bunkr.init_time, &current_time, root.to_vec(), proof_path, otp_hash)?;
        validate_password(bunkr.current_hash, &password_hash)?;
        bunkr.last_accessed = current_time as u32;
        bunkr.current_hash = password_hash.as_slice().try_into().unwrap();
        msg!("Withdrawal successful");

        Ok(())
    }
    pub fn freeze_non_fungible(ctx: Context<FreezeNonFungible>) -> Result<()> {

        let cpi_accounts = Approve{
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
            delegate: ctx.accounts.delegate.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        approve(cpi_context, 1)?;

        let signer = ctx.accounts.signer.key();
        
        let seeds = &[
            b"testvault",
            signer.as_ref(),
            &[*ctx.bumps.get("delegate").unwrap()]
        ];
        let delegate_seeds = &[&seeds[..]];

        invoke_signed(
        &freeze_delegated_account(
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
    Ok(())
    }

    pub fn thaw_non_fungible(ctx: Context<ThawNonFungible>) -> Result<()> {
        let signer = ctx.accounts.signer.key();

        let seeds = &[
            b"testvault",
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

    pub fn lock_fungible(ctx: Context<LockFungible>, amount: u64) -> Result<()> {
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.from_associated_token_account.to_account_info(),
            to: ctx.accounts.to_associated_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        let raw_amount = amount * (10_usize.pow(ctx.accounts.token_mint.decimals as u32)) as u64;
        anchor_spl::token::transfer(cpi_context, raw_amount)?;

    
        
        Ok(())
    }

    pub fn unlock_fungible(ctx: Context<UnlockFungible>, amount: u64) -> Result<()> {

        let cpi_accounts = Transfer {
            from: ctx.accounts.from_associated_token_account.to_account_info(),
            to: ctx.accounts.to_associated_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        };
        
        let signer = ctx.accounts.signer.key();
        let seeds = &[
            b"testvault",
            signer.as_ref(),
            &[*ctx.bumps.get("vault").unwrap()]
        ];

        let vault_seeds = &[&seeds[..]];

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(vault_seeds);

        let raw_amount = amount * (10_usize.pow(ctx.accounts.token_mint.decimals as u32)) as u64;
        anchor_spl::token::transfer(cpi_context, raw_amount)?;

        Ok(())
    }

    pub fn hash_test(ctx: Context<HashTest>, hash_list: Vec<HashTuple>) -> Result<()> {
        //let mut hash: Hash = anchor_lang::solana_program::hash::hash(word.as_bytes());
        // msg!("Initial Hash: {}", bs58::encode(hash).into_string());

        // for _ in 0..amount {
        //     hash = anchor_lang::solana_program::hash::hash(hash.as_ref());
        // }

        for hash_tuple in hash_list.iter() {
            msg!("Hash: {:?}", hash_tuple.hash);
            //msg!("Sibling Index: {}", hash_tuple.sibling_index.to_string());
        }
        // msg!("Final Hash: {}", hash.to_string());
        // msg!("Supplied Hash: {}", bs58::encode(&final_hash).into_string());
        // assert!(hash.to_bytes() == final_hash, "Hashes do not match");

        Ok(())
    }

    pub fn hash_extend_test(ctx: Context<HashTest>, hash: Vec<u8>, hash_extended: Vec<u8>) -> Result<()> {
        let mut hash: Hash = anchor_lang::solana_program::hash::Hash::new_from_array(hash.try_into().unwrap());
        msg!("Initial Hash: {}", hash.to_string());
        let extended_hash = anchor_lang::solana_program::hash::extend_and_hash(&hash, b"1");
        msg!("Extended Hash: {}", extended_hash.to_string());



        Ok(())
    }

}


fn validate_root(init_time: u32, current_time: &i64, root: Vec<u8>, proof_path: Vec<HashTuple>, hash: Vec<u8>) -> Result<()> {
    assert!(root.len() == 32, "Root must be 32 bytes");
    assert!(hash.len() == 32, "Hash must be 32 bytes");
    assert!(proof_path.len() == 20, "Proof path must be 32 hashes");
    for hash_tuple in proof_path.iter() {
        assert!(hash_tuple.hash.len() == 32, "Hash must be 32 bytes");
    }


    let root = Hash::new_from_array(root.try_into().unwrap());
    let hash: Hash = anchor_lang::solana_program::hash::Hash::new_from_array(hash.try_into().unwrap());
    let time_interval = (current_time - init_time as i64) / 30;
    msg!("Time Interval: {}", time_interval.to_string());
    let extended_hashes: [Hash; 2] = [anchor_lang::solana_program::hash::extend_and_hash(&hash, &time_interval.to_string().as_bytes()), anchor_lang::solana_program::hash::extend_and_hash(&hash, &(time_interval - 1).to_string().as_bytes())];
    msg!("Extended Hashes: {:?}", extended_hashes);
    let mut calculated_roots: Vec<Hash> = vec![];

    for hash in extended_hashes {

        let mut calculated_hash: Hash = hash.clone();

        for hash_tuple in proof_path.iter() {
            match hash_tuple.sibling_index {
                0 => {
                    calculated_hash = anchor_lang::solana_program::hash::extend_and_hash(&Hash::new_from_array(hash_tuple.hash.try_into().unwrap()), &calculated_hash.to_bytes());
                },
                1 => {
                    calculated_hash = anchor_lang::solana_program::hash::extend_and_hash(&calculated_hash, &hash_tuple.hash);
                },
                _ => {
                    panic!("Invalid Sibling Index")
                }
            }
        }
        calculated_roots.push(calculated_hash);
    }
    msg!("Calculated Roots: {:?}", calculated_roots);

    assert!(calculated_roots.contains(&root), "Root Mismatch");

    Ok(())
}

fn validate_time(last_accessed: u32, current_time: &i64) -> Result<()> {
    assert!(current_time - last_accessed as i64 > 30, "Account has been accessed in the last 30 seconds");
    Ok(())
}

fn validate_password(on_chain_password: [u8; 32], password: &Vec<u8>) -> Result<()> {
    let password_to_check = anchor_lang::solana_program::hash::hash(password.as_slice());
    assert!(on_chain_password == password_to_check.to_bytes(), "Password Mismatch");
    Ok(())
}






#[derive(Accounts)]
pub struct InitBunkr<'info> {
    #[account(init, seeds=[b"bunkr", signer.key().as_ref()], bump , payer=signer, space = 300)]
    pub bunkr: Account<'info,Bunkr>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CloseBunkr<'info> {
    #[account(mut, close = signer, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    pub bunkr: Account<'info,Bunkr>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
pub struct TestWithdraw<'info> {
    #[account(mut, seeds=[b"bunkr", signer.key().as_ref()], bump)]
    bunkr: Account<'info, Bunkr>,
    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct TimeTest<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct FreezeNonFungible<'info> {
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

    #[account(mut, seeds=[b"testvault", signer.key().as_ref()], bump)]
    delegate: Account<'info, Bunkr>,

    token_program: Program<'info, Token>,
    /// CHECK intstruction will fail if wrong program is supplied
    token_metadata_program: AccountInfo<'info>
}

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

    #[account(mut, seeds=[b"testvault", signer.key().as_ref()], bump)]
    delegate: Account<'info, Bunkr>,

    token_program: Program<'info, Token>,
    /// CHECK intstruction will fail if wrong program is supplied
    token_metadata_program: AccountInfo<'info>

}

#[derive(Accounts)]
pub struct LockFungible<'info> {
    #[account(
        mut, 
        associated_token::mint = token_mint, 
        associated_token::authority = signer
    )]
    from_associated_token_account: Account<'info, TokenAccount>,
    token_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        associated_token::mint = token_mint,
        associated_token::authority = vault,
        payer = signer
    )]
    to_associated_token_account: Account<'info, TokenAccount>,
    /// CHECK instruction will fail if wrong edition is supplied
    #[account(mut)]
    signer: Signer<'info>,

    #[account(mut, seeds=[b"testvault", signer.key().as_ref()], bump)]
    vault: Account<'info, Bunkr>,

    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>
}

#[derive(Accounts)]
pub struct UnlockFungible<'info> {
    #[account(
        mut, 
        associated_token::mint = token_mint, 
        associated_token::authority = vault
    )]
    from_associated_token_account: Account<'info, TokenAccount>,
    token_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        associated_token::mint = token_mint,
        associated_token::authority = signer,
        payer = signer
    )]
    to_associated_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    signer: Signer<'info>,

    #[account(mut, seeds=[b"testvault", signer.key().as_ref()], bump)]
    vault: Account<'info, Bunkr>,

    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>


}

#[derive(Accounts)]
pub struct HashTest<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}



#[account]
pub struct Bunkr {
    pub name: String,
    pub withdraw_address: Pubkey,
    pub init_time: u32,
    pub last_accessed: u32,
    pub root: [u8; 32],
    pub current_hash: [u8; 32],
    pub final_hash: [u8; 32],
    pub current_reset_hash: [u8; 32],
    pub final_reset_hash: [u8; 32],
    pub shadow_drive_space: String
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


#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct HashTuple {
    pub hash: [u8; 32],
    pub sibling_index: u8
}
