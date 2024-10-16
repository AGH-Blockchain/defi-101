use anchor_lang::prelude::*;

use crate::Transfer;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub token_lp: Pubkey,
}

pub fn transfer<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Transfer<'info>>,
    amount: u64,
) -> Result<()> {
    #[allow(deprecated)]
    anchor_spl::token_2022::transfer(ctx, amount)
}

#[error_code]
pub enum ErrorCodes {
    #[msg("Token X must be less than Token Y")]
    TokenXGreaterThanTokenY,
    #[msg("Invalid authority")]
    InvalidAuthority,
}
