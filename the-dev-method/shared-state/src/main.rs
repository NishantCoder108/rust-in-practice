use std::sync::Mutex;
use std::thread;

/* 2) Shared a value with mulitple threads */

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            /*value is moved from first thread and still trying to access it ,so that reason giving error */
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); /*When we use join, it will wait for thread finished their job. It will check sequentially, might be other thread have already worked done. */
    }

    println!("Result: {}", *counter.lock().unwrap());
    /*We can't move ownership in multiple threads */
}

/*
/* 1) Using Mutex - in single threaded context */
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
*/
