use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{mint_to, transfer_checked, MintTo, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::Vault;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"vault"], bump)]
    pub vault: Box<Account<'info, Vault>>,

    #[account(mut,
        associated_token::mint = mint_a,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub depositor_account_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut,
        associated_token::mint = mint_b,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub depositor_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(init_if_needed,
        payer = signer,
        associated_token::mint = mint_lp,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub depositor_account_lp: Box<InterfaceAccount<'info, TokenAccount>>,

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

pub fn deposit_token_a(ctx: &Context<Deposit>, amount: u64) -> Result<()> {
    let transfer_a = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.depositor_account_a.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.vault_a.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        },
    );
    transfer_checked(transfer_a, amount, ctx.accounts.mint_a.decimals)?;
    Ok(())
}

pub fn deposit_token_b(ctx: &Context<Deposit>, amount: u64) -> Result<()> {
    let transfer_b = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.depositor_account_b.to_account_info(),
            mint: ctx.accounts.mint_b.to_account_info(),
            to: ctx.accounts.vault_b.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        },
    );
    transfer_checked(transfer_b, amount, ctx.accounts.mint_b.decimals)?;
    Ok(())
}

pub fn mint_lp_tokens(ctx: &Context<Deposit>, amount: u64) -> Result<()> {
    let seeds = &[b"vault".as_ref(), &[ctx.bumps.vault]];
    let signer = &[&seeds[..]];

    let mint_to_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint_lp.to_account_info(),
            to: ctx.accounts.depositor_account_lp.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
    )
    .with_signer(signer);
    mint_to(mint_to_ctx, amount)?;

    Ok(())
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    deposit_token_a(&ctx, amount)?;
    deposit_token_b(&ctx, amount)?;
    mint_lp_tokens(&ctx, amount)?;

    Ok(())
}
