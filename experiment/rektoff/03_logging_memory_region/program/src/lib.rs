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

    // Allow any non-empty instruction_data, not just 0.
    match instruction_data.get(0) {
        Some(0) => log_memory_regions(instruction_data),
        Some(1) => attempt_uaf(),
        Some(2) => attempt_buffer_overflow(),
        Some(3) => dangling_pointer(),
        Some(_) => {
            msg!("Unrecognized instruction value: {}", instruction_data[0]);
            Err(ProgramError::InvalidInstructionData)
        }
        None => {
            msg!("No instruction data provided.");
            Err(ProgramError::InvalidInstructionData)
        }
    }
    // 0 => log_memory_regions(instruction_data),
    // 1 => attempt_uaf(),
    // 2 => attempt_buffer_overflow(),
    // _ => Err(ProgramError::InvalidInstructionData),
    // }
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

fn attempt_uaf() -> ProgramResult {
    msg!("Attempting UAF");

    let x = Box::new(5);
    msg! {"Value before free: {}", *x};
    let raw_x = Box::into_raw(x);

    unsafe {
        drop(Box::from_raw(raw_x));
        msg!("Value after free: {}", *raw_x);
    }

    Ok(())
}

fn attempt_buffer_overflow() -> ProgramResult {
    let mut buffer = [0u8; 5];
    let not_in_buffer = 56789;

    unsafe {
        let ptr = buffer.as_mut_ptr();

        for i in 0..6 {
            *ptr.add(i) = i as u8;
            msg!("writing {} at:          {:p}", i, ptr.add(i));
        }
    }

    msg!("");
    msg!("buffer address:        {:p}", &buffer);
    msg!("not_in_buffer address: {:p}", &not_in_buffer);

    msg!("buffer: {:?}", buffer);
    msg!("not_in_buffer: {}", not_in_buffer);

    Ok(())
}

fn dangling_pointer() -> ProgramResult {
    msg!("Attempting Dangling Pointer");

    let ptr: *const u8;
    {
        let array = [0xAA, 0xBB, 0xCC, 0xDD];
        ptr = array.as_ptr();
        msg!("Pointer created at: {:p}", ptr);
        // Use read_volatile to force the compiler to not optimize out the initialization
        unsafe {
            core::ptr::read_volatile(ptr);
        }
    } // `array` is dropped here

    // Read after drop
    unsafe {
        msg!("Reading dangling pointer:");
        // volatile read to ensure it actually happens despite optimizations
        let val = core::ptr::read_volatile(ptr);
        msg!("Value: {:x}", val);
    }

    Ok(())
}
