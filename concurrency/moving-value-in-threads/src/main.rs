use std::thread;

fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    let first_handler = thread::spawn(move || {
        println!("Vec value : {vec:?}");
    });

    let x = 10;

    //it's closure
    let print_x = || {
        println!("{}", x);
    };
    print_x();

    first_handler.join().unwrap() //join() , it will wait till thread execution is completed
}

/*
 * || { ... } → this is a closure
 * A closure is basically a small anonymous function that can capture variables from its surrounding or parent environment.
 *
 * One thing living longer than another - outlive means
 *
 * move : we use move keyword to own the ownership
 *
 */
