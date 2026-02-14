use anchor_lang::prelude::*;

declare_id!("GCagCXvTjjcH4LrmPoCnVAypBq2yhnnU3McSF8sQFUqC");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Changed data to: {}!", data);
        ctx.accounts.new_account.data = data;
        Ok(())
    }
}

// #[derive(Accounts)]
// pub struct Initialize {}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
// #[derive(InitSpace)]
pub struct NewAccount {
    data: u64,
    // #[max_len(30)]
    // name: String,
}
