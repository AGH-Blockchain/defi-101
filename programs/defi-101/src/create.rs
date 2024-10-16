use std::cmp::Ordering;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenInterface},
};

use crate::{Error, Vault};

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault"],
        bump
    )]
    pub vault: Box<Account<'info, Vault>>,

    #[account(mint::token_program = token_program)]
    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    #[account(mint::token_program = token_program)]
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = vault,
        mint::freeze_authority = vault,
        mint::token_program = token_program
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn create(ctx: Context<Create>) -> Result<()> {
    let mint_a = ctx.accounts.mint_a.key().to_string();
    let mint_b = ctx.accounts.mint_b.key().to_string();

    require!(
        mint_a.cmp(&mint_b) == Ordering::Less,
        Error::TokenXGreaterThanTokenY
    );

    Ok(())
}
