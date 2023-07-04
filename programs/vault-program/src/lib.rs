use {crate::instructions::*, crate::states::*, anchor_lang::prelude::*};

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;


declare_id!("BunKrGBXdGxyTLjvE44eQXDuKY7TyHZfPu9bj2Ugk5j2");

#[program]
pub mod vault_program {

    use crate::states::Bunkr;

    use super::*;

    pub fn init_bunkr(ctx: Context<InitBunkr> , init_bunkr_data: InitBunkrData ) -> Result<()> {
        init_bunkr::handler(ctx, init_bunkr_data)
    }

    pub fn change_withdrawal_address(ctx: Context<ChangeWithdrawalAddress>) -> Result<()> {
        let bunkr = &mut ctx.accounts.bunkr;
        change_withdrawal_address::handler(ctx)
    }

    pub fn close_bunkr(_ctx: Context<CloseBunkr>) -> Result<()> {
        close_bunkr::handler(_ctx)
    }

    pub fn freeze_non_fungible(ctx: Context<FreezeNonFungible>) -> Result<()> {
        freeze_non_fungible::handler(ctx)
    }

    pub fn thaw_non_fungible(ctx: Context<ThawNonFungible>) -> Result<()> {
        thaw_non_fungible::handler(ctx)
    }

    pub fn lock_pnft(ctx: Context<LockPNFT>) -> Result<()> {
        lock_pnft::handler(ctx)
    }

    pub fn unlock_pnft(ctx: Context<UnlockPNFT>) -> Result<()> {
        unlock_pnft::handler(ctx)
    }
    pub fn lock_fungible(ctx: Context<LockFungible>, amount: u64) -> Result<()> {
        lock_fungible::handler(ctx, amount)
    }

    pub fn unlock_fungible(ctx: Context<UnlockFungible>, amount: u64) -> Result<()> {
        unlock_fungible::handler(ctx, amount)
    }

    pub fn test_withdraw(_ctx: Context<TestWithdraw>) -> Result<()> {
        test_withdraw::handler(_ctx)
    }
}







