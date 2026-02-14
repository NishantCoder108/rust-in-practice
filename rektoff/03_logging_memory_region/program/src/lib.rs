#[warn(unexpected_cfgs)]
use solana_program::{
    account_info::AccountInfo,
    entrypoint::{ProgramResult, entrypoint},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Declare the program's entry point
entrypoint!(process_instruction);

// Program entry point's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Log a message to the Solana runtime
    msg!("Hello, world!");
    msg!("This is a simple logging program for learning.");
    msg!("Program executed successfully!");

    if instruction_data.is_empty() {
        msg!("Hello, world!");
        msg!("This is a simple logging program for learning.");
        msg!("Program executed successfully!");
        msg!("Instruction data is empty for this logging program.");
        return Ok(());
    }

    match instruction_data[0] {
        0 => log_memory_regions(instruction_data),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn log_memory_regions(instruction_data: &[u8]) -> ProgramResult {
    // Vec is good because it has stack and heap components
    let vec = vec![0x11, 0x22, 0x33];

    // Args should be in Input data 0x400000000
    msg!("instr args   = {:p}", instruction_data);

    // Vec data should be on heap in 0x300000000
    msg!("vec heap     = {:p}", &vec[0]);

    // Vec struct should be on stack in 0x200000000
    msg!("vec stack    = {:p}", &vec);

    // Static var should be in 0x100000000
    // To access data stored in a static variable from the client side (off-chain), you can't directly access on-chain program memory.
    // However, you can expose the data via logs, which the client can then read from the transaction's log messages.
    static IN_RO: u64 = 0x0123_4567_89ab_cdef;

    // Log both the address and value so that a client can read them from the transaction logs.
    msg!("IN_RO static address = {:p}", &IN_RO);
    msg!("IN_RO static value   = {:#x}", IN_RO);

    // Instructions should be in the text segment at 0x000000000
    msg!(
        "process_instruction address: {:p}",
        process_instruction as *const ()
    );
    msg!(
        "log_memory_regions address: {:p}",
        log_memory_regions as *const ()
    );

    Ok(())
}
