use anchor_lang::prelude::*;

mod create;
mod deposit;
mod initialize;
mod misc;
mod swap;
mod withdraw;

pub use anchor_spl::token_2022::Transfer;
pub use misc::*;

use create::*;
use deposit::*;
use initialize::*;
use swap::*;
use withdraw::*;

declare_id!("8aDyKXbYswuR4Czq8q6PCtuiYTqSrNn776z4LLHA47uu");

#[program]
pub mod defi_101 {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        create::create(ctx)
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::initialize(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        deposit::deposit(ctx, amount)
    }

    pub fn swap(ctx: Context<Swap>, amount: i64) -> Result<()> {
        swap::swap(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw::withdraw(ctx)
    }
}
