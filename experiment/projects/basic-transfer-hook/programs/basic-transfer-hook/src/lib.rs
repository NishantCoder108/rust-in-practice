use anchor_lang::prelude::*;
use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    // Remove the problematic derive usage of Mint and TokenAccount from token_interface
    // Use AccountInfo for CPI-only accounts (see error context and suggested usage!)
    token_interface::TokenInterface,
};
use spl_tlv_account_resolution::state::ExtraAccountMetaList;
use spl_transfer_hook_interface::instruction::{ExecuteInstruction, TransferHookInstruction};

declare_id!("FH3jX9TuYHaSXRSo4AGEYxAVTTA1aDqXymQhjnQAPG86");

#[program]
pub mod basic_transfer_hook {
    use super::*;

    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>,
    ) -> Result<()> {
        // The `addExtraAccountsToInstruction` JS helper function resolving incorrectly
        let account_metas = vec![];

        // calculate account size
        let account_size: u64 = ExtraAccountMetaList::size_of(account_metas.len()).unwrap() as u64;

        // calculate minimum required lamports
        let lamports = Rent::get()?.minimum_balance(account_size as usize);

        let mint = ctx.accounts.mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"extra-account-metas",
            &mint.as_ref(),
            &[ctx.bumps.extra_account_meta_list],
        ]];

        // create ExtraAccountMetaList account
        create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                CreateAccount {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.extra_account_meta_list.to_account_info(),
                },
            )
            .with_signer(signer_seeds),
            lamports,
            account_size,
            ctx.program_id,
        )?;

        // initialize ExtraAccountMetaList account with extra accounts
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &account_metas,
        )
        .unwrap();

        Ok(())
    }

    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        msg!("Hello Transfer Hook!");

        const MAX: u64 = 50 * 1_000_000_000;
        if amount > MAX {
            return err!(MyError::AmountTooBig);
        }
        Ok(())
    }

    // fallback instruction handler as workaround to anchor instruction discriminator check
    pub fn fallback<'info>(
        program_id: &Pubkey,
        accounts: &'info [AccountInfo<'info>],
        data: &[u8],
    ) -> Result<()> {
        let instruction = TransferHookInstruction::unpack(data).unwrap();

        // match instruction discriminator to transfer hook interface execute instruction
        // token2022 program CPIs this instruction on token transfer
        match instruction {
            TransferHookInstruction::Execute { amount } => {
                let amount_bytes = amount.to_le_bytes();
                __private::__global::transfer_hook(program_id, accounts, &amount_bytes)
            }
            _ => return Err(ProgramError::InvalidInstructionData.into()),
        }
    }
}

/// Account struct for initializing the ExtraAccountMetaList PDA.
/// Note: All accounts (except payer) are just CPI-only (no Anchor account parsing)
#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        mut,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: AccountInfo<'info>,

    /// CHECK: Just checked as a generic account, since we're using this Mint for seed only
    pub mint: AccountInfo<'info>,

    /// CHECK: Not using interface parsing for CPI-only token program
    pub token_program: AccountInfo<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

/// Order of accounts matters -- Use AccountInfo for all except owner (UncheckedAccount).
#[derive(Accounts)]
pub struct TransferHook<'info> {
    /// CHECK: Source token account, just pass as AccountInfo for CPI
    pub source_token: AccountInfo<'info>,

    /// CHECK: Mint account, CPI only
    pub mint: AccountInfo<'info>,

    /// CHECK: Destination token account
    pub destination_token: AccountInfo<'info>,

    /// CHECK: Source token owner, can be SystemAccount or PDA owned by another program
    pub owner: UncheckedAccount<'info>,

    /// CHECK: ExtraAccountMetaList PDA
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
}

#[error_code]
pub enum MyError {
    #[msg("The amount is too big")]
    AmountTooBig,
}
