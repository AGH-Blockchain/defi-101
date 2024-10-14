use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_2022::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}
};

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub amount_a: u64,
    pub amount_b: u64,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init,
        payer = signer, 
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault", mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

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

    #[account(init,
        payer = signer,
        associated_token::mint = mint_a,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(init,
        payer = signer,
        associated_token::mint = mint_b,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_b: Box<InterfaceAccount<'info, TokenAccount>  >,

    #[account(mint::token_program = token_program)]
    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    #[account(mint::token_program = token_program)]
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    send_wanted_tokens(&ctx, amount)?;
    Ok(())
}

pub fn send_wanted_tokens(ctx: &Context<Deposit>, amount: u64) -> Result<()> {
    let transfer_a = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
        from: ctx.accounts.depositor_account_a.to_account_info(),
        mint: ctx.accounts.mint_a.to_account_info(),
        to: ctx.accounts.vault_a.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    });
    let transfer_b = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.depositor_account_b.to_account_info(),
            mint: ctx.accounts.mint_b.to_account_info(),
            to: ctx.accounts.vault_b.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        },);
    
    transfer_checked(transfer_a, amount, ctx.accounts.mint_a.decimals)?;
    transfer_checked(transfer_b, amount, ctx.accounts.mint_b.decimals)?;

    Ok(())
}
