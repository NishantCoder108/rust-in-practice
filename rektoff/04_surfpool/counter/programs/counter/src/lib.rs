#[allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("ADNojXa6H4bY9cbPPpiQamcU5gcwiSEik6dqbWn9QMa4");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
