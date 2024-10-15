use anchor_lang::prelude::*;

mod create;
mod deposit;
mod swap;
mod withdraw;

use create::*;
use deposit::*;
use swap::*;
use withdraw::*;
declare_id!("GwAUakR2tZWd5WQmvXfvSZJHjGwbcrnyr4AAkhXpCCMx");

#[error_code]
pub enum Error {
    #[msg("Token X must be less than Token Y")]
    TokenXGreaterThanTokenY,
    #[msg("Invalid authority")]
    InvalidAuthority,
}

#[program]
pub mod defi_101 {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        deposit::deposit(ctx, amount)
    }

    pub fn create(ctx: Context<Create>) -> Result<()> {
        create::create(ctx)
    }

    pub fn swap(ctx: Context<Swap>, amount: i64) -> Result<()> {
        swap::swap(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw::withdraw(ctx)
    }
}

#[account]
#[derive(InitSpace)]
pub struct Vault {}
