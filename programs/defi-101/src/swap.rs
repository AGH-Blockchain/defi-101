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

pub fn swap(ctx: Context<Swap>, amount: i64) -> Result<()> {
    send_wanted_tokens(&ctx, amount)?;
    Ok(())
}

pub fn send_wanted_tokens(ctx: &Context<Swap>, amount: i64) -> Result<()> {
    let seeds = &[b"vault".as_ref(), &[ctx.bumps.vault]];
    let signer = &[&seeds[..]];

    // TODO: fit on stack.

    let (take, send) = if amount > 0 {
        let take_a = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.depositor_account_a.to_account_info(),
                mint: ctx.accounts.mint_a.to_account_info(),
                to: ctx.accounts.vault_a.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        );

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

        (take_a, send_b)
    } else {
        let take_b = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.depositor_account_b.to_account_info(),
                mint: ctx.accounts.mint_b.to_account_info(),
                to: ctx.accounts.vault_b.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        );

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

        (take_b, send_a)
    };

    let amount = amount.checked_abs().unwrap() as u64;

    transfer_checked(take, amount, ctx.accounts.mint_a.decimals)?;
    transfer_checked(send, amount, ctx.accounts.mint_b.decimals)?;

    Ok(())
}
