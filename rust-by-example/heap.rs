use std::mem;

fn dump_str(label: &str, s: &str) {
    println!(
        "{:<16} | data_ptr={:p} len={:<2} | value={:?}",
        label,
        s.as_ptr(),
        s.len(),
        s
    );
}

fn dump_string(label: &str, s: &String) {
    println!(
        "{:<16} | data_ptr={:p} len={:<2} cap={:<2} | value={:?}",
        label,
        s.as_ptr(),
        s.len(),
        s.capacity(),
        s
    );
}

fn main() {
    // ============================================================
    // A) Box<&str> — copies the fat pointer (ptr + len)
    // ============================================================

    let s: &str = "hello";
    // `s` is a stack variable holding a fat pointer (data ptr + len).
    // The bytes "hello" live in static read-only memory (rodata).

    let b: Box<&str> = Box::new(s);
    // `b` is a stack variable holding a Box pointer (thin pointer).
    // The Box allocates space on the heap to store a COPY of the &str fat pointer.
    // The string bytes themselves are NOT copied or moved.

    println!("== Box<&str>: where the bytes live ==");
    dump_str("s", s);
    dump_str("*b", *b);

    println!("\n== Box<&str>: prove the &str was copied ==");
    println!(
        "{:<16} | s.data_ptr == (*b).data_ptr ? {}",
        "data_ptr",
        s.as_ptr() == (*b).as_ptr()
    );
    println!(
        "{:<16} | s.len == (*b).len ? {}",
        "len",
        s.len() == (*b).len()
    );
    println!(
        "{:<16} | &s == &*b ? {}",
        "address",
        (&s as *const _) == (&*b as *const _)
    );

    assert_eq!(s.as_ptr(), (*b).as_ptr());
    assert_eq!(s.len(), (*b).len());
    assert_ne!((&s as *const _), (&*b as *const _));

    println!("\n== Box<&str>: pointer locations ==");
    println!("{:<16} | &s   (stack) = {:p}", "&s", &s);
    println!("{:<16} | &*b  (heap)  = {:p}", "&*b", &*b);
    println!("{:<16} | &b   (stack) = {:p}", "&b", &b);

    println!("\n== Box<&str>: sizes ==");
    println!(
        "{:<16} | size_of::<&str>()      = {}",
        "&str",
        mem::size_of::<&str>()
    );
    println!(
        "{:<16} | size_of::<Box<&str>>() = {}",
        "Box<&str>",
        mem::size_of::<Box<&str>>()
    );

    // ============================================================
    // B) Box<String> — moves the String handle (ptr + len + cap)
    // ============================================================

    let s2: String = "hello".to_string();
    // `String` is a stack value holding (data ptr + len + cap) pointing to heap bytes.

    println!("\n== Box<String>: before move ==");
    dump_string("s2", &s2);

    let b2: Box<String> = Box::new(s2);
    // Ownership of the String handle is MOVED into the box.
    // The handle now lives on the heap; the bytes stay where they are.

    println!("\n== Box<String>: after move ==");
    dump_string("*b2", &b2);

    println!("\n== Box<String>: pointer locations ==");
    println!("{:<16} | &*b2 (heap)  = {:p}", "&*b2", &*b2);
    println!("{:<16} | &b2  (stack) = {:p}", "&b2", &b2);

    println!("\n== Box<String>: sizes ==");
    println!(
        "{:<16} | size_of::<String>()      = {}",
        "String",
        mem::size_of::<String>()
    );
    println!(
        "{:<16} | size_of::<Box<String>>() = {}",
        "Box<String>",
        mem::size_of::<Box<String>>()
    );

    // ============================================================
    // C) Box<str> — the box owns the string bytes
    // ============================================================

    let b3: Box<str> = "hello".into();
    // The bytes "hello" are allocated on the heap and owned by the box itself.

    println!("\n== Box<str>: where the bytes live ==");
    dump_str("b3", &b3);

    println!("\n== Box<str>: pointer locations ==");
    println!("{:<16} | &b3  (stack) = {:p}", "&b3", &b3);
    println!("{:<16} | data_ptr     = {:p}", "b3.as_ptr()", b3.as_ptr());

    println!("\n== Box<str>: sizes ==");
    println!(
        "{:<16} | size_of::<Box<str>>() = {}",
        "Box<str>",
        mem::size_of::<Box<str>>()
    );

    // Note: exact addresses and stack layout vary by compiler/build;
    // the pointer relationships are what matter.
}
