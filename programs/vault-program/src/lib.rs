use {crate::instructions::*, anchor_lang::prelude::*};

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

declare_id!("BunKrGBXdGxyTLjvE44eQXDuKY7TyHZfPu9bj2Ugk5j2");

#[program]
pub mod vault_program {

    use super::*;

    pub fn init_bunkr(ctx: Context<InitBunkr> , init_bunkr_data: InitBunkrData ) -> Result<()> {
        init_bunkr::handler(ctx, init_bunkr_data)
    }

    pub fn change_withdrawal_address(ctx: Context<ChangeWithdrawalAddress>, authentication_object: AuthenticationObject) -> Result<()> {
        let bunkr = &mut ctx.accounts.bunkr;
        authentication::handler(bunkr, authentication_object)?;
        change_withdrawal_address::handler(ctx)
    }

    pub fn reset_root(ctx: Context<ResetRoot>, reset_root_object: ResetRootObject) -> Result<()> {
        reset_root::handler(ctx, reset_root_object)
    }

    pub fn confirm_root(ctx: Context<ConfirmRoot>, confirm_root_object: AuthenticationObject) -> Result<()> {
        confirm_root::handler(ctx, confirm_root_object)
    }

    pub fn close_bunkr(ctx: Context<CloseBunkr>) -> Result<()> {
        close_bunkr::handler(ctx)
    }

    pub fn freeze_non_fungible(ctx: Context<FreezeNonFungible>) -> Result<()> {
        freeze_non_fungible::handler(ctx)
    }

    pub fn thaw_non_fungible(ctx: Context<ThawNonFungible>, authentication_object: AuthenticationObject) -> Result<()> {
        let bunkr = &mut ctx.accounts.bunkr;
        authentication::handler(bunkr, authentication_object)?;
        thaw_non_fungible::handler(ctx)
    }

    pub fn lock_fungible(ctx: Context<LockFungible>, amount: u64) -> Result<()> {
        lock_fungible::handler(ctx, amount)
    }

    pub fn unlock_fungible(ctx: Context<UnlockFungible>, amount: u64, authentication_object: AuthenticationObject) -> Result<()> {
        let bunkr = &mut ctx.accounts.bunkr;
        authentication::handler(bunkr, authentication_object)?;
        unlock_fungible::handler(ctx, amount)
    }

    pub fn test_withdraw(ctx: Context<TestWithdraw>, authentication_object: AuthenticationObject) -> Result<()> {
        let bunkr = &mut ctx.accounts.bunkr;
        authentication::handler(bunkr, authentication_object)?;
        test_withdraw::handler(ctx)
    }
}







