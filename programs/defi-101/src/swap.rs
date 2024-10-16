use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::Vault;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"vault"], bump)]
    pub vault: Box<Account<'info, Vault>>,

    #[account(init_if_needed,
        payer = signer,
        associated_token::mint = mint_a,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub depositor_account_a: InterfaceAccount<'info, TokenAccount>,
    #[account(init_if_needed,
        payer = signer,
        associated_token::mint = mint_b,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub depositor_account_b: InterfaceAccount<'info, TokenAccount>,

    #[account(init_if_needed,
        payer = signer,
        associated_token::mint = mint_a,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(init_if_needed,
        payer = signer,
        associated_token::mint = mint_b,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mint::token_program = token_program)]
    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    #[account(mint::token_program = token_program)]
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

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
        TransferChecked {
            from: ctx.accounts.depositor_account_a.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.vault_a.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        },
    );
    transfer_checked(take_a, amount, ctx.accounts.mint_a.decimals)?;
    Ok(())
}

fn take_token_b(ctx: &Context<Swap>, amount: u64) -> Result<()> {
    let take_b = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.depositor_account_b.to_account_info(),
            mint: ctx.accounts.mint_b.to_account_info(),
            to: ctx.accounts.vault_b.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        },
    );
    transfer_checked(take_b, amount, ctx.accounts.mint_b.decimals)?;
    Ok(())
}

fn send_token_a(ctx: &Context<Swap>, amount: u64) -> Result<()> {
    let seeds = &[b"vault".as_ref(), &[ctx.bumps.vault]];
    let signer = &[&seeds[..]];
    let send_a = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.vault_a.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.depositor_account_a.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
    )
    .with_signer(signer);
    transfer_checked(send_a, amount, ctx.accounts.mint_a.decimals)?;
    Ok(())
}

fn send_token_b(ctx: &Context<Swap>, amount: u64) -> Result<()> {
    let seeds = &[b"vault".as_ref(), &[ctx.bumps.vault]];
    let signer = &[&seeds[..]];
    let send_b = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.vault_b.to_account_info(),
            mint: ctx.accounts.mint_b.to_account_info(),
            to: ctx.accounts.depositor_account_b.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
    )
    .with_signer(signer);
    transfer_checked(send_b, amount, ctx.accounts.mint_b.decimals)?;
    Ok(())
}

pub fn swap(ctx: Context<Swap>, amount: i64) -> Result<()> {
    if amount > 0 {
        swap_a_for_b(&ctx, amount as u64)
    } else {
        swap_b_for_a(&ctx, amount.checked_abs().unwrap() as u64)
    }
}
