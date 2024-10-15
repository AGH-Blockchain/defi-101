use anchor_lang::prelude::*;

mod create;
mod deposit;

declare_id!("GwAUakR2tZWd5WQmvXfvSZJHjGwbcrnyr4AAkhXpCCMx");

#[error_code]
pub enum Error {
    #[msg("Token X must be less than Token Y")]
    TokenXGreaterThanTokenY,
    #[msg("Invalid authority")]
    InvalidAuthority,
}

use create::*;
use deposit::*;

#[program]
pub mod defi_101 {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        deposit::deposit(ctx, amount)
    }

    pub fn create(ctx: Context<Create>) -> Result<()> {
        create::create(ctx)
    }
}

#[account]
#[derive(InitSpace)]
pub struct Vault {}
