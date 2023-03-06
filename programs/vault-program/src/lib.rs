use {crate::instructions::*, anchor_lang::prelude::*};

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

declare_id!("3PHnbmYZU1zVmD3rrLbkFTQonrRFj2GwRZwnLHJunAfL");

#[program]
pub mod vault_program {

    use super::*;

    pub fn init_bunkr(ctx: Context<InitBunkr> , init_bunkr_data: InitBunkrData ) -> Result<()> {
        init_bunkr::handler(ctx, init_bunkr_data)
    }

    pub fn close_bunkr(ctx: Context<CloseBunkr>) -> Result<()> {
        close_bunkr::handler(ctx)
    }

    pub fn freeze_non_fungible(ctx: Context<FreezeNonFungible>) -> Result<()> {
        freeze_non_fungible::handler(ctx)
    }

    pub fn thaw_non_fungible(ctx: Context<ThawNonFungible>) -> Result<()> {
        thaw_non_fungible::handler(ctx)
    }

    pub fn lock_fungible(ctx: Context<LockFungible>, amount: u64) -> Result<()> {
        lock_fungible::handler(ctx, amount)
    }

    pub fn unlock_fungible(ctx: Context<UnlockFungible>, amount: u64) -> Result<()> {
        unlock_fungible::handler(ctx, amount)
    }

    pub fn test_withdraw(ctx: Context<TestWithdraw>, authentication_object: AuthenticationObject) -> Result<()> {
        let bunkr = &mut ctx.accounts.bunkr;
        authentication::handler(bunkr, authentication_object)?;
        test_withdraw::handler(ctx)
    }
}







