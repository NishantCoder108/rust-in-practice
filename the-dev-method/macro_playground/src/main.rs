// fn main() {
//     // println!("Hello, world!");
//     let v: Vec<u32> = vec![1, 2, 3];
// }
/* ------ Declarative Macros ------- */
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();
}
