use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let data = Arc::clone(&counter);

        let handler = thread::spawn(move || {
            let mut counter = data.lock().unwrap();
            *counter += 1;
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("Counter : {:?}", *counter.lock().unwrap());
    // println!("Hello, world!");

    // let data = Mutex::new(0);

    // {
    //     let mut lock_data = data.lock().unwrap();

    //     println!("LOCK DATA: {lock_data:?}");

    //     *lock_data += 1;
    // }
    // println!("Data: {data:?}");
}

//when goes out of scope then automatically lock value end
