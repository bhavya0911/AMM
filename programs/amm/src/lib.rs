use anchor_lang::prelude::*;

mod constants;
mod state;
mod contexts;
use contexts::*;
mod error;
mod helpers;

declare_id!("6oge3B2hEP2gToUeQLAXNwtfe6m92V4rmtENw7XN3e7f");

#[program]
pub mod week2_3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, fee, authority)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64, expiration: i64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y, expiration)
    }

    pub fn swap(ctx: Context<Swap>, is_x: bool, amount: u64, min: u64, expiration: i64) -> Result<()> {
        ctx.accounts.swap(is_x, amount, min, expiration)
    }

    pub fn lock(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.lock()
    }

    pub fn unlock(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.unlock()
    }
}