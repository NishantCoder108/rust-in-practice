use std::thread;

fn main() {
    let first_thread = thread::spawn(|| {
        println!("Printing from first thread");
    });

    let second_thread = thread::spawn(|| {
        println!("Printing from second thread");

        for num in 0..5 {
            println!("Second thread Num: {num:?}");
        }
    });

    for num in 0..5 {
        println!("Printing from main thread. Num: {num:?}");
    }

    first_thread.join().unwrap();
    second_thread.join().unwrap();
}

/*
* We have 1 in-built threads, that we say Main threads. but we can create multiple thread in rust using `threads::spawn`

- Most important for creating threads is to waiting till task has been not completed - `first_thread.join().unwrap()`
*/
