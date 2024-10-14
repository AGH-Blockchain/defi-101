use anchor_lang::prelude::*;

declare_id!("GwAUakR2tZWd5WQmvXfvSZJHjGwbcrnyr4AAkhXpCCMx");

#[program]
pub mod defi_101 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
