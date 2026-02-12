use std::sync::Mutex;

/* Using Mutex - in single threaded context */
fn main() {
    let n = Mutex::new(9);

    {
        let mut num = n.lock().unwrap();
        *num *= 12;
        // The lock is automatically released (unlocked) here, when 'num' goes out of scope.
    } // <-- unlock happens here

    // When printing a Mutex with Debug, you'll see fields like 'data' and 'poisoned'.
    // 'poisoned' indicates if a panic occurred while the Mutex was locked. If a thread panics while holding the lock,
    // the Mutex is considered poisoned to signal possible data inconsistency. Here, 'poisoned: false' means no panic occurred.
    println!("n = {n:?}"); // e.g., Mutex { data: 108, poisoned: false, .. }

    /* Understand with steps:
    Mutex : locks the data -> do stuff using data -> If done with data -> unlock the data
    1. first we acquire the lock. i.e. `m.lock()`
    2. now we can access the data and mutate it
    3. unlock the data if it requires.

    */
}
