static GREETING_OUTER: &str = "Hello, universe!";

fn main() {
    greet();
    println!("{}", GREETING);
    static GREETING: &str = "Hello, world!";
}

fn greet() {
    // println!("{}", GREETING);
    println!("{}", GREETING_OUTER)
}

/*
-> `println!("{}", GREETING);` This line will give error

-> GREETING is not available in greet and global scope. So, we can't access it
*/
