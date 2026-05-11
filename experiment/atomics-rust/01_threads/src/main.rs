use std::thread;

fn main() {
    let t1 = thread::spawn(hello);
    let t2 = thread::spawn(hello);

    println!("hello from the main thread");

    t1.join().unwrap();
    t2.join().unwrap();
}

fn hello() {
    println!("Hello from thread");
    let id = thread::current().id();
    println!("This is hello current id: {:?}", id);
}
