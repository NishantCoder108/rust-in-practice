use std::sync::Mutex;

fn main() {
    // println!("Hello, world!");

    let data = Mutex::new(0);

    {
        let mut lock_data = data.lock().unwrap();

        println!("LOCK DATA: {lock_data:?}");

        *lock_data += 1;
    }
    println!("Data: {data:?}");
}
