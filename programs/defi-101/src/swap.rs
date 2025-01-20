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

fn swap_a_for_b(
    ctx: &Context<Swap>,
    amount: u64,
    reserve_in_before: u64,
    reserve_out_before: u64,
) -> Result<()> {
    let amount_out = calculate_out(reserve_in_before, reserve_out_before, amount)?;

    take_token_a(ctx, amount)?;
    send_token_b(ctx, amount_out)?;
    Ok(())
}

fn swap_b_for_a(
    ctx: &Context<Swap>,
    amount: u64,
    reserve_a_before: u64,
    reserve_b_before: u64,
) -> Result<()> {
    let amount_out = calculate_out(reserve_b_before, reserve_a_before, amount)?;
    take_token_b(ctx, amount)?;
    send_token_a(ctx, amount_out)?;
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
    let reserve_a = ctx.accounts.vault_a.amount;
    let reserve_b = ctx.accounts.vault_b.amount;

    if amount > 0 {
        swap_a_for_b(&ctx, amount as u64, reserve_a, reserve_b)
    } else {
        swap_b_for_a(
            &ctx,
            amount.checked_abs().unwrap() as u64,
            reserve_b,
            reserve_a,
        )
    }
}

fn calculate_out(reserve_token_in: u64, reserve_token_out: u64, amount_in: u64) -> Result<u64> {
    let k = reserve_token_in * reserve_token_out;
    let reserve_in_after = reserve_token_in + amount_in;
    let reserve_out_after = k / reserve_in_after;

    Ok(reserve_token_out - reserve_out_after)
}

#[test]
fn test_calculate_out() {
    let amount_out = calculate_out(100, 100, 10).unwrap();
    assert_eq!(amount_out, 9);
}
