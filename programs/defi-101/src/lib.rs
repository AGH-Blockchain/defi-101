use anchor_lang::prelude::*;

mod deposit;

declare_id!("GwAUakR2tZWd5WQmvXfvSZJHjGwbcrnyr4AAkhXpCCMx");

use deposit::*;

#[program]
pub mod defi_101 {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        deposit::deposit(ctx, amount)
    }
}
