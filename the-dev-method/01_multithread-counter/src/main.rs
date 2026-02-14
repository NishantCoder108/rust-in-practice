use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    let timer_start = Instant::now();
    for _ in 0..20 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 100000;
        });

        handlers.push(handler);
    }

    /* If we don't use for loop `join` method so it will not wait, that reason counter value can give error data value
     logs :  Counter : Mutex { data: 1900000, poisoned: false, .. } ==> 2000000
    */
    for handler in handlers {
        handler.join().unwrap(); //it will synchronously run and it will check task is done or not. It will act on that. But some thread might be done tasks before handler waited.
    }

    let duration = timer_start.elapsed();
    println!("Counter : {:?} ==> 2000000", counter);
    println!("Total duration: {:?}", duration);

    /* Logs---
    Counter : Mutex { data: 2000000, poisoned: false, .. } ==> 2000000
    Total duration: 329.917µs
     */

    timer_for_counter_without_thread(); //Experiment counter without thread how much it will take time
}
/*
1. Counter that will be increment to 20 lakh
2. We will create thread and distribute to 20 thread and every will increment to 1lakh
3. At the end to 20 threads, I will receive expected result.
*/

fn timer_for_counter_without_thread() {
    let mut count = 0;

    let start = Instant::now(); //timer start
    while count < 2000000 {
        count += 100000;
    }

    let duration = start.elapsed();

    println!("Total duration : {:?}", duration);
    println!("Counter : {}", count);
}
