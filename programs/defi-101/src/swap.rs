use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{TokenAccount, TokenInterface},
};

use crate::{transfer, Transfer, Vault};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"vault"], bump)]
    pub vault: Box<Account<'info, Vault>>,

    #[account(mut,
        associated_token::mint = vault.token_a,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub depositor_account_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut,
        associated_token::mint = vault.token_b,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub depositor_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut,
        associated_token::mint = vault.token_a,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut,
        associated_token::mint = vault.token_b,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_b: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

fn swap_a_for_b(ctx: &Context<Swap>, amount: u64) -> Result<()> {
    take_token_a(ctx, amount)?;
    send_token_b(ctx, amount)?;
    Ok(())
}

fn swap_b_for_a(ctx: &Context<Swap>, amount: u64) -> Result<()> {
    take_token_b(ctx, amount)?;
    send_token_a(ctx, amount)?;
    Ok(())
}

fn take_token_a(ctx: &Context<Swap>, amount: u64) -> Result<()> {
    let take_a = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.depositor_account_a.to_account_info(),
            to: ctx.accounts.vault_a.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        },
    );
    transfer(take_a, amount)?;
    Ok(())
}

fn take_token_b(ctx: &Context<Swap>, amount: u64) -> Result<()> {
    let take_b = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.depositor_account_b.to_account_info(),
            to: ctx.accounts.vault_b.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        },
    );
    transfer(take_b, amount)?;
    Ok(())
}

fn send_token_a(ctx: &Context<Swap>, amount: u64) -> Result<()> {
    let seeds = &[b"vault".as_ref(), &[ctx.bumps.vault]];
    let signer = &[&seeds[..]];
    let send_a = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_a.to_account_info(),
            to: ctx.accounts.depositor_account_a.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
    )
    .with_signer(signer);
    transfer(send_a, amount)?;
    Ok(())
}

#[inline(never)]
fn send_token_b(ctx: &Context<Swap>, amount: u64) -> Result<()> {
    let seeds = &[b"vault".as_ref(), &[ctx.bumps.vault]];
    let signer = &[&seeds[..]];
    let send_b = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_b.to_account_info(),
            to: ctx.accounts.depositor_account_b.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
    )
    .with_signer(signer);
    transfer(send_b, amount)?;

    Ok(())
}

pub fn swap(ctx: Context<Swap>, amount: i64) -> Result<()> {
    if amount > 0 {
        swap_a_for_b(&ctx, amount as u64)
    } else {
        swap_b_for_a(&ctx, amount.checked_abs().unwrap() as u64)
    }
}
