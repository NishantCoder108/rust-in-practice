#![allow(dead_code, unused_variables)]

use std::{future::Future, io::BufRead};

fn main() {
    // println!("Hello, world!");

    // let x = foo1();

    println!("Hello, world!");
    let read_from_terminal = std::thread::spawn(move || {
        let mut x = std::io::Stdin::lock(&std::io::stdin());

        for line in x.lines() {

            // do something on user input
        }
    });

    let read_from_network = std::thread::spawn(move || {
        let mut x = std::net::TcpListener::bind("0.0.0.0:8080").unwrap();
        while let Ok(stream) = x.accept() {
            let handle = std::thread::spawn(move || {
                // handle_connection(stream);
            });

            let x = foo2();
        }
    });

    // let network = read_from_network();
    // let terminal = read_from_terminal();
    // select! {

    // stream <- network await = {

    // }
    // line <- terminal.await =>

    // };

    // let x = f0o2();
}
async fn foo1() -> usize {
    println!("foo");
    0
}

fn foo2() -> impl Future<Output = usize> {
    async { 0 }
}

#[expect(unused_variables)]
fn example1() -> i32 {
    let x = {
        return 5;
    };
}
// In contrast, the following asynchronous function assigns a Future<Output = i32> to x, and only returns 5 when x is .awaited:

async fn example() -> i32 {
    let x = async {
        return 5;
    };

    x.await
}
