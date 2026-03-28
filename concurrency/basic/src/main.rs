use std::thread;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    let main_thread_id = thread::current().id();
    println!("Hello from the main thread: {main_thread_id:?}");
    // thread::sleep(std::time::Duration::from_millis(100));

    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();

    println!("average: {average}");

    t1.join().unwrap();
    t2.join().unwrap(); //It will wait for the thread to finish
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
