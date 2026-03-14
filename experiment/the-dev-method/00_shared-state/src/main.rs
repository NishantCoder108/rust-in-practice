use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

/* Multiple ownership with Multiple threads */
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    /*
    - ARC -> It use atomic reference counter pointer.
    - Atomic ? -> Read, Add , Write will happen in one instruction. so other thread must wait.

    - ARC vs RC -> Only difference is atomicity. so we safely use on multiple threads.
    - We can also say, RC have not `SEND` trait but ARC have, because of atomicity.
    - `Send` is a marker trait that means a type can be safely transferred to another thread

    - Race condition: It happen when we use some thing like RC at multiple threads.
    - suppose we have data, first thread is reading and other thread is also read data before first thread updating the data.
    - so, now, both thread are adding stuff, now value would be 2 but it will not, it will be 1. thats a race condition.


    - One cause can be happen that is deadlock, its a logical error.
    - one is waiting for other thread and other thread is waiting for one thread. so it will stuck forevery
    */
}
/*
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


//-------------------------------------------------

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


//-----------------------------------------

/*--Multiple ownership */
/*
   RC - Reference Counted pointer (Multiple ownership at single threads)
 -> It use for multiple owner of same data.
 -> We can use for mulitple ownership of data.
 -> We can't use `RC` for multiple ownership with multiple threads because of `SEND` traits is not implemented
 -> It is made for mulitple ownership with single threads
 -> RC stores internally `data and reference_counter`. When we write `clone`, it will increase reference counter and when we `drop`, it will decrease the counter.
 -> RC have not `SEND` trait so it can't cross threads
 -> We can use `ARC` instead of `RC`. It will have same things but It has also SEND traits, so it has multiple owner with multiple threads.
 -> Why Rc is not Send but Arc is?”
 -> Because of Arc use atomic reference counting and Rc use non-atomic reference cunting.

*/
fn main() {
    multiple_ownership();
}

fn multiple_ownership() {
    let one_owner = Rc::new(String::from("Multiple Ownership"));
    let second_owner = Rc::clone(&one_owner);
    let third_owner = Rc::clone(&one_owner);

    println!("One Owner : {:?}", one_owner);
    println!("Second Owner : {:?}", second_owner);
    println!("Third Owner : {}", third_owner);

    /*
    -> It is directly returning the value

    One Owner : "Multiple Ownership"
    Second Owner : "Multiple Ownership"
    Third Owner : Multiple Ownership
    */
}
*/
