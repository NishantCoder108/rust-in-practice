use std::sync::Mutex;

fn main() {
    // println!("Hello, world!");

    let data = Mutex::new(0);

    println!("Data: {data:?}");
}
