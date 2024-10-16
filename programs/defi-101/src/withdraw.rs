use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{burn, transfer_checked, Burn, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::Vault;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"vault"], bump)]
    pub vault: Box<Account<'info, Vault>>,

    #[account(mut,
        associated_token::mint = mint_a,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub depositor_account_a: InterfaceAccount<'info, TokenAccount>,
    #[account(mut,
        associated_token::mint = mint_b,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub depositor_account_b: InterfaceAccount<'info, TokenAccount>,

    #[account(init_if_needed,
        payer = signer,
        associated_token::mint = mint_lp,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub depositor_account_lp: InterfaceAccount<'info, TokenAccount>,

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

    #[account(mut,
        mint::decimals = 9,
        mint::authority = vault,
        mint::freeze_authority = vault,
        mint::token_program = token_program
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,

    #[account(mint::token_program = token_program)]
    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    #[account(mint::token_program = token_program)]
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

fn send_token_a(ctx: &Context<Withdraw>, amount_a: u64) -> Result<()> {
    let seeds = &[b"vault".as_ref(), &[ctx.bumps.vault]];
    let signer = &[&seeds[..]];
    let transfer_a = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.vault_a.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.depositor_account_a.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
    )
    .with_signer(signer);
    transfer_checked(transfer_a, amount_a, ctx.accounts.mint_a.decimals)
}

fn send_token_b(ctx: &Context<Withdraw>, amount_b: u64) -> Result<()> {
    let seeds = &[b"vault".as_ref(), &[ctx.bumps.vault]];
    let signer = &[&seeds[..]];
    let transfer_b = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.vault_b.to_account_info(),
            mint: ctx.accounts.mint_b.to_account_info(),
            to: ctx.accounts.depositor_account_b.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
    )
    .with_signer(signer);
    transfer_checked(transfer_b, amount_b, ctx.accounts.mint_b.decimals)
}

fn burn_lp_tokens(ctx: &Context<Withdraw>, amount: u64) -> Result<()> {
    let burn_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.mint_lp.to_account_info(),
            from: ctx.accounts.depositor_account_lp.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        },
    );
    burn(burn_ctx, amount)
}

pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    let amount = ctx.accounts.depositor_account_lp.amount as u128;
    let total_lp = ctx.accounts.mint_lp.supply as u128;
    let amount_a = amount * ctx.accounts.vault_a.amount as u128 / total_lp;
    let amount_b = amount * ctx.accounts.vault_b.amount as u128 / total_lp;

    send_token_a(&ctx, amount_a as u64)?;
    send_token_b(&ctx, amount_b as u64)?;
    burn_lp_tokens(&ctx, amount as u64)?;

    Ok(())
}
