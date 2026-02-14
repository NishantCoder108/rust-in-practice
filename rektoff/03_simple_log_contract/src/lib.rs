use solana_program::{
    account_info::AccountInfo,
    entrypoint::{ProgramResult, entrypoint},
    msg,
    pubkey::Pubkey,
};

// Declare the program's entry point
entrypoint!(process_instruction);

// Program entry point's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // Log a message to the Solana runtime
    msg!("Hello, world!");
    msg!("This is a simple logging program for learning.");
    msg!("Program executed successfully!");

    msg!("Simple logs contract");

    Ok(())
}
