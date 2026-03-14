use anchor_lang::prelude::borsh::de;
#[allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("ADNojXa6H4bY9cbPPpiQamcU5gcwiSEik6dqbWn9QMa4");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("  Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // ctx.accounts.count.count += 1;
        // msg!("  Counter: {}", ctx.accounts.count.count);


        let a_count = &mut ctx.accounts.count;

        if a_count.count < 10 {
                a_count.count += 1;
        } else {
            return Err(CounterError::CounterOverflow.into());
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


#[derive(Accounts)]
pub struct Increment<'info> {

    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        init_if_needed,
        payer = sender,
        seeds = [b"count".as_ref(), sender.key().as_ref()],
        bump,
        space = 8 + Count::INIT_SPACE
    )]
    pub count: Account<'info, Count>,
    pub system_program: Program<'info, System>,
}



#[account]
#[derive(InitSpace)]
pub struct Count {
    pub count: u64,
}



#[error_code]
pub enum CounterError {
    #[msg("Counter overflow")]
    CounterOverflow,
}


//surfpool run deployment --env localnet -u


//solana get account counteraddress

/*
-not giving -u -> supervisor that looks graet
 : localhost:8488 port -> supervised execution


*/