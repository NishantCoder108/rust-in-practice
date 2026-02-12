const FIRST: &str = "FIRST_CONST"; // string in .rodata, not nm due to inlining
static SECOND: &str = "SECOND_STATIC"; // ptr in .data, string in .rodata
static mut THIRD: u32 = 1337; // in .data (writable)
static mut FOURTH: [u8; 16] = [0; 16]; // in .bss (zero initialised data)

// fn main() {
//     static FIFTH: &str = "FIFTH_LOCAL_STATIC"; // ptr in .data, string in .rodata
//     let sixth = "SIXTH_STACK";   // string in .rodata, pointer on stack

//     #[allow(static_mut_refs)] // refs to mutating statics denied by default
//     unsafe {
//         println!(
//             "{} | {} | {} | {:?} | {} | {}",
//             FIRST,
//             SECOND,
//             THIRD,
//             FOURTH,
//             FIFTH,
//             sixth
//         );
//     }
// }

/*--------- UAF -> Use After Free memory-------- */
fn uaf() {
    let num = Box::new(3);

    /*
     * Convert into raw pointer
     * Ownership goes away from Rust
     * It will not remove from memory automatically
     */
    let num_raw_pointer = Box::into_raw(num);

    // println!("{num_raw_pointer:#?}"); //0x00000001045f1b00

    unsafe {
        /*
         * Ownership back to Rust
         * It will remove from memory using keyword `drop()`
         * Convert it back
         */
        // println!("Before Raw Pointer Value: {}", *num_raw_pointer);
        let back_num_raw_pointer = Box::from_raw(num_raw_pointer);

        drop(back_num_raw_pointer);

        // println!("{num_raw_pointer:?}");
        println!("Raw Pointer Value : {}", *num_raw_pointer)
    }
}

/*--------Buffer Overflow -> End of allocated memory use-------- */

fn main() {
    let mut buffer = [0u8; 5]; // Stack-allocated buffer (5 bytes)
    let not_in_buffer = 56789; // Stack variable just after the buffer

    unsafe {
        let ptr = buffer.as_mut_ptr(); // Raw pointer to start of buffer

        // 🚨 UB: writing 6 bytes into a 5-byte buffer (1 byte overflow)
        for i in 0..6 {
            *ptr.add(i) = i as u8;
        }
    }

    println!("buffer: {:?}", buffer); // Expected: [0, 1, 2, 3, 4]
    println!("not_in_buffer: {}", not_in_buffer); // Will this still be 56789?
}
