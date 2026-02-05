use anchor_lang::prelude::*;

declare_id!("GCagCXvTjjcH4LrmPoCnVAypBq2yhnnU3McSF8sQFUqC");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
